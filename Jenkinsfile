pipeline {
  agent any
  stages {
    stage('Welcome') {
      steps {
        sh 'echo hi' 
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build' 
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

  }
}
