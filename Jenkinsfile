pipeline {
    agent {
        docker { image 'rust:latest-alpine' }
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
    }
}