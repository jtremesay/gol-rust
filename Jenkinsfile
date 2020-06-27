pipeline {
    agent {
        docker { image 'rust:alpine' }
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
    }
}