import React from 'react'
import {BrowserRouter, Switch, Route} from 'react-router-dom'
import App from './App'
import Signin from './auth/Signin';
import Private from './core/Private';
import PrivateRoute from './auth/PrivateRoute';
import Admin from './core/Admin';
import AdminRoute from './auth/AdminRoute';
import BankAccounts from './core/BankAccounts';
import BankCards from './core/BankCards';
import BankFunds from './core/BankFunds';
import BankLoans from './core/BankLoans';

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
                <Route path="/bankloans" component={BankLoans} />

                <PrivateRoute path="/private" exact component={Private} />
                <AdminRoute path="/admin" exact component={Admin}></AdminRoute>

                
            </Switch>
        </BrowserRouter>
    )
};


export default Routes;