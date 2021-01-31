import React from 'react'
import {BrowserRouter, Switch, Route} from 'react-router-dom'
import App from './App'
import Signup from './auth/Signup';
import Signin from './auth/Signin';
import Private from './core/Private';
import PrivateRoute from './auth/PrivateRoute';
import Admin from './core/Admin';
import AdminRoute from './auth/AdminRoute';
import ProfesorRoute from './auth/ProfesorRoute'
import Profesor from './core/Profesor'
import StudentRoute from './auth/StudentRoute';
import Student from './core/Student';
import BankAccounts from './core/BankAccounts';
import BankCards from './core/BankCards';
import BankFunds from './core/BankFunds';
// import Units from './core/Units';
// import Play from './quiz/Play';
// import QuizSummary from './quiz/QuizSummary';
const Routes = () => {
    return(
        <BrowserRouter>
            <Switch>
                <Route exact path="/" component={App} />
                {/* <Route path="/signup" component={Signup} /> */}
                <Route path="/signin" component={Signin} />
                <Route path="/bankaccounts" component={BankAccounts} />
                <Route path="/bankcards" component={BankCards} />
                <Route path="/bankfunds" component={BankFunds} />

                <PrivateRoute path="/private" exact component={Private} />
                <AdminRoute path="/admin" exact component={Admin}></AdminRoute>
                <ProfesorRoute path="/profesor" exact component={Profesor}></ProfesorRoute>
                <StudentRoute path="/student" exact component={Student}></StudentRoute>
                {/* <Units path="/testunits" exact component={Units}></Units> */}
                {/* <Route path="/play/quiz/:name" exact component={Play} />
                <Route path="/play/quizSummary" exact component={QuizSummary} /> */}
                
            </Switch>
        </BrowserRouter>
    )
};


export default Routes;