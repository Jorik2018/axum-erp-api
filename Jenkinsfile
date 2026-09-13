pipeline {
    agent any

    options {
        timestamps()
        disableConcurrentBuilds()
    }

    environment {
        APP_NAME = 'axum-treasury-api'

        SERVICE_ID = 'axum-treasury-api'
        SERVICE_NAME = 'AXUM Treasury API'
        SERVICE_DESCRIPTION = 'Axum Treasury API'

        PORT = '8085'

        DEPLOY_DIR = 'D:\\microservices\\axum-treasury-api'
    
    CARGO_EXE_NAME = 'axum-erp-api.exe'
    EXE_NAME = 'axum-treasury-api.exe'
        PYTHON_HOME = 'C:\\Tools\\Python312'
        SERVICE_MANAGER = 'D:\\wildfly\\bin\\service_manager.py'

        VAULT_ADDR = 'http://localhost:8200'
    }

    stages {


stage('Verify Vault Token') {
    steps {
        withCredentials([
            string(
                credentialsId: 'VAULT_TOKEN',
                variable: 'VAULT_TOKEN'
            )
        ]) {
            powershell '''
                Write-Host "=========================================="
                Write-Host "Verifying Vault token"
                Write-Host "=========================================="

                $headers = @{
                    "X-Vault-Token" = $env:VAULT_TOKEN
                }

                try {
                    $response = Invoke-WebRequest `
                        -Uri "http://127.0.0.1:8200/v1/auth/token/lookup-self" `
                        -Headers $headers `
                        -Method GET `
                        -UseBasicParsing

                    Write-Host "HTTP Status:" $response.StatusCode
                    Write-Host "Vault response:"
                    Write-Host $response.Content
                }
                catch {
                    Write-Host "HTTP request failed"

                    if ($_.Exception.Response) {
                        try {
                            Write-Host "HTTP Status:" ([int]$_.Exception.Response.StatusCode)
                        }
                        catch {}
                    }

                    Write-Host "Vault response:"

                    if ($_.ErrorDetails -and $_.ErrorDetails.Message) {
                        Write-Host $_.ErrorDetails.Message
                    }
                    else {
                        Write-Host $_.Exception.Message
                    }

                    exit 1
                }
            '''
        }
    }
}
        stage('Rust Environment') {
            steps {
                bat '''
                    echo ==========================================
                    echo Rust environment
                    echo ==========================================

                    whoami

                    where rustup
                    where rustc
                    where cargo

                    rustup --version
                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Python Environment') {
            steps {
                bat '''
                    echo ==========================================
                    echo Python environment
                    echo ==========================================

                    if not exist "%PYTHON_HOME%\\python.exe" (
                        echo ERROR: Python not found at:
                        echo %PYTHON_HOME%\\python.exe
                        exit /B 1
                    )

                    "%PYTHON_HOME%\\python.exe" --version

                    if not exist "%SERVICE_MANAGER%" (
                        echo ERROR: Service manager not found:
                        echo %SERVICE_MANAGER%
                        exit /B 1
                    )

                    echo Service manager:
                    echo %SERVICE_MANAGER%
                '''
            }
        }

        stage('Update Rust') {
            steps {
                bat '''
                    set RUSTUP_HOME=C:\\Users\\Administrador.WIN-5UFR8AED4T8\\.rustup
                    set CARGO_HOME=C:\\Users\\Administrador.WIN-5UFR8AED4T8\\.cargo
                    set PATH=%CARGO_HOME%\\bin;%PATH%

                    rustup update stable
                    rustup default stable

                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Build') {
            steps {
                bat '''
                    echo ==========================================
                    echo Building Rust application
                    echo ==========================================

                    cargo build --release

                    if errorlevel 1 (
                        echo ERROR: Cargo build failed
                        exit /B 1
                    )

                    if not exist "target\\release\\%CARGO_EXE_NAME%" (
                        echo ERROR: Executable was not generated
                        echo Expected:
                        echo target\\release\\%CARGO_EXE_NAME%
                        exit /B 1
                    )

                    echo Build completed successfully.
                '''
            }
        }

        stage('Test') {
            steps {
                bat '''
                    echo ==========================================
                    echo Running tests
                    echo ==========================================

                    cargo test --release

                    if errorlevel 1 (
                        echo ERROR: Tests failed
                        exit /B 1
                    )
                '''
            }
        }

        stage('Prepare Deploy') {
            steps {
                bat '''
                    echo ==========================================
                    echo Preparing deployment
                    echo ==========================================

                    if not exist "%DEPLOY_DIR%" (
                        mkdir "%DEPLOY_DIR%"
                    )

                    if errorlevel 1 (
                        echo ERROR: Could not create deploy directory
                        exit /B 1
                    )
                '''
            }
        }

        stage('Stop Service') {
            steps {
                bat '''
                    echo ==========================================
                    echo Stopping service
                    echo ==========================================

                    "%PYTHON_HOME%\\python.exe" "%SERVICE_MANAGER%" stop "%SERVICE_ID%"
                '''
            }
        }

        stage('Deploy') {
            steps {
                bat '''
                    echo ==========================================
                    echo Deploying %APP_NAME%
                    echo ==========================================

                    copy /Y ^
                        "target\\release\\%CARGO_EXE_NAME%" ^
                        "%DEPLOY_DIR%\\%EXE_NAME%"

                    if errorlevel 1 (
                        echo ERROR copying executable
                        exit /B 1
                    )

                    echo Deployed:
                    echo %DEPLOY_DIR%\\%EXE_NAME%
                '''
            }
        }
stage('Service Manager Help') {
    steps {
        bat '''
            "%PYTHON_HOME%\\python.exe" "%SERVICE_MANAGER%" install --help
        '''
    }
}

stage('Configure Service') {
    steps {
        withCredentials([
            string(
                credentialsId: 'VAULT_TOKEN',
                variable: 'VAULT_TOKEN'
            )
        ]) {
            bat '''
                echo ==========================================
                echo Configuring Windows service
                echo ==========================================

                "%PYTHON_HOME%\\python.exe" "%SERVICE_MANAGER%" install ^
                    "%SERVICE_ID%" ^
                    "%DEPLOY_DIR%" ^
                    --name "%SERVICE_NAME%" ^
                    --description "%SERVICE_DESCRIPTION%" ^
                    --type rust ^
                    --env "PORT=%PORT%" ^
                    --env "VAULT_TOKEN=%VAULT_TOKEN%" ^
                    --env "JWT_PUBLIC_KEY=D:\\java\\publicKey.pem" ^
                    --executable "%EXE_NAME%"

                if errorlevel 1 (
                    echo ERROR: Service configuration failed
                    exit /B 1
                )
            '''
        }
    }
}

stage('Reinstall Windows Service') {
    steps {
        bat '''
            echo ==========================================
            echo Reinstalling Windows service
            echo ==========================================

            sc stop "%SERVICE_ID%" >nul 2>&1
            sc delete "%SERVICE_ID%" >nul 2>&1

            timeout /t 2 /nobreak >nul

            cd /d "%DEPLOY_DIR%"

            service.exe install

            if errorlevel 1 (
                echo ERROR: Could not install service
                exit /B 1
            )

            sc qc "%SERVICE_ID%"
        '''
    }
}

stage('Start Service') {
    steps {
        bat '''
            echo ==========================================
            echo Windows service configuration
            echo ==========================================

            sc qc "%SERVICE_ID%"

            echo ==========================================
            echo Starting service
            echo ==========================================

            sc start "%SERVICE_ID%"

            if errorlevel 1 (
                echo.
                echo ==========================================
                echo ERROR: Could not start service
                echo ==========================================

                sc query "%SERVICE_ID%"

                echo.
                echo WinSW files:
                dir "%DEPLOY_DIR%"

                exit /B 1
            )
        '''
    }
}

stage('Verify Service') {
    steps {
        bat '''
            echo ==========================================
            echo Checking Windows service
            echo ==========================================

            sc query "%SERVICE_ID%"

            sc query "%SERVICE_ID%" | findstr /I "RUNNING"

            if errorlevel 1 (
                echo ERROR: Service is not running.
                exit /B 1
            )

            echo Service is RUNNING.

            echo ==========================================
            echo Checking Rust API port
            echo ==========================================

            netstat -ano | findstr ":%PORT%"

            if errorlevel 1 (
                echo ERROR: Rust API is not listening on port %PORT%
                exit /B 1
            )

            echo Rust API is listening on port %PORT%.
        '''
    }
}
    }

    post {
        success {
            echo '=========================================='
            echo 'AXUM Treasury API deployed successfully'
            echo '=========================================='
        }

        failure {
            echo '=========================================='
            echo 'AXUM Treasury API deployment FAILED'
            echo '=========================================='
        }

        always {
archiveArtifacts(
    artifacts: "target/release/${env.CARGO_EXE_NAME}",
    fingerprint: true,
    allowEmptyArchive: true
)
        }
    }
}