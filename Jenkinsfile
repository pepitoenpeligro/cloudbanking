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
          gitStatusWrapper(credentialsId: '55e0da58a952da11a338cbb088c960566d994584', gitHubContext: 'Status', description: 'Validating') {
              sh 'cargo test'
          }
      }
    }

    

  }
}
