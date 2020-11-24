pipeline {
  agent any
  stages {
    stage('Welcome') {
      steps {
        sh 'echo hi again x4' 
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


      stage('EZPZ Updates') {
      steps {
          gitStatusWrapper(credentialsId: '17498e2c9e5cd7667e9d5dc0ecaaabd23c92291d', gitHubContext: 'Status', description: 'Validating') {
              sh 'cargo test'
          }
      }
    }

    

  }
}
