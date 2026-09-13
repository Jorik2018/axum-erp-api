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

        DEPLOY_DIR = 'D:\\services\\axum-treasury-api'

        EXE_NAME = 'axum-treasury-api.exe'

        PYTHON = 'python'

        SERVICE_SCRIPT = 'D:\\jenkins\\scripts\\service.py'
    }

    stages {

        stage('Environment') {
            steps {
                bat '''
                    echo ==========================================
                    echo Rust environment
                    echo ==========================================

                    where cargo
                    cargo --version

                    where rustc
                    rustc --version

                    echo ==========================================
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

                    if not exist "target\\release\\%EXE_NAME%" (
                        echo ERROR: Executable was not generated
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
                '''
            }
        }

        stage('Stop Service') {
            steps {
                bat '''
                    %PYTHON% "%SERVICE_SCRIPT%" stop ^
                        --service-id "%SERVICE_ID%"
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
                        "target\\release\\%EXE_NAME%" ^
                        "%DEPLOY_DIR%\\%EXE_NAME%"

                    if errorlevel 1 (
                        echo ERROR copying executable
                        exit /B 1
                    )
                '''
            }
        }

        stage('Configure Service') {
            steps {
                bat '''
                    %PYTHON% "%SERVICE_SCRIPT%" install ^
                        --service-id "%SERVICE_ID%" ^
                        --destination "%DEPLOY_DIR%" ^
                        --service-name "%SERVICE_NAME%" ^
                        --description "%SERVICE_DESCRIPTION%" ^
                        --app-type rust ^
                        --port %PORT% ^
                        --executable "%EXE_NAME%"
                '''
            }
        }

        stage('Start Service') {
            steps {
                bat '''
                    %PYTHON% "%SERVICE_SCRIPT%" start ^
                        --service-id "%SERVICE_ID%"
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
                '''
            }
        }
    }

    post {
        success {
            echo '=========================================='
            echo 'Warrant API deployed successfully'
            echo '=========================================='
        }

        failure {
            echo '=========================================='
            echo 'Warrant API deployment FAILED'
            echo '=========================================='
        }

        always {
            archiveArtifacts(
                artifacts: 'target/release/axum-treasury-api.exe',
                fingerprint: true,
                allowEmptyArchive: true
            )
        }
    }
}