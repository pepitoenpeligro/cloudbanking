import React, { Component } from 'react';
import { Route, Redirect } from 'react-router-dom';
import { isAuth } from './helpers';

const StudentRoute = ({ component: Component, ...rest }) => (
    <Route
        {...rest}
        render={props =>
            isAuth() && isAuth().role==='student' ? (
                <Component {...props} />
            ) : (
                <Redirect
                    to={{
                        pathname: '/student',
                        state: { from: props.location }
                    }}
                />
            )
        }
    ></Route>




);

export default StudentRoute;
