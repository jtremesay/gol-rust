pipeline {
    agent {
        docker { image 'rust:latest' }
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
    }
}