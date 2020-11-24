pipeline {
  agent any
  stages {
    stage('Welcome') {
      steps {
        sh 'echo hi again x2' 
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
