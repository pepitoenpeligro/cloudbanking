import React, {Fragment} from 'react'
import {Link, withRouter} from 'react-router-dom'
import {isAuth, signout} from '../auth/helpers'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';

import './Layout.css';
const Layout = ({children, match, history}) => {

    const isActive = path => {
        if(match.path === path){
            return {color:'#003D8A'}
        }else{
            return {color: '#fff'};
        }
    }

    const nav = () => (

        <nav id="navbar">
        <ul className="navbar navbar-expand-lg nav nav-tab bg-primary " >


            <li className="nav-item ">
                <Link to="/" className="nav-link"   style={isActive('/')}  >
                <img src="imgs/icon5.png" height="30" alt="logo"></img>
                   </Link>
            </li>


            {
                isAuth() && isAuth().role==='admin' && (
                    <Fragment>
                         <li className="nav-item" >
                        <Link to="/admin" className="nav-link" style={isActive('/admin')}>
                            Admin</Link>
                    </li>
                    </Fragment>
                )
            }

            { !isAuth() &&(
                <Fragment>
                     {/* <li className="nav-item" >
                        <Link to="/signup" className="nav-link" style={isActive('/signup')}>
                            Signup</Link>
                    </li>  */}

                    <li className="nav-item" >
                        <Link to="/signin" className="nav-link" style={isActive('/signin')}>
                            Signin</Link>
                    </li>
                </Fragment>
            )}



            { isAuth()  && (
       
                            <Fragment>
                                <li className="nav-item" >
                                    <Link to="/bankaccounts" className="nav-link" style={isActive('/bankaccounts')}>
                                        Accounts</Link>
                                </li>

                                <li className="nav-item" >
                                    <Link to="/bankcards" className="nav-link" style={isActive('/bankcards')}>
                                        Cards</Link>
                                </li>

                                <li className="nav-item" >
                                    <Link to="/bankfunds" className="nav-link" style={isActive('/bankfunds')}>
                                        Funds</Link>
                                </li>

                                <li className="nav-item" >
                                    <Link to="/bankloans" className="nav-link" style={isActive('/bankloans')}>
                                        Loans</Link>
                                </li>
                            </Fragment>
                        )}


            { isAuth() && isAuth().role=== 'student' && (
                <Fragment>
                    <li className="nav-item" >
                        <Link to="/student" className="nav-link" style={isActive('/student')}>
                            Student</Link>
                    </li>
                </Fragment>
            )}


            { isAuth() && isAuth().role=== 'profesor' && (
                <Fragment>
                    <li className="nav-item" >
                        <Link to="/profesor" className="nav-link" style={isActive('/profesor')}>
                            Profesor</Link>
                    </li>
                </Fragment>
            )}

            {isAuth() && (<div className="ml-auto">
                    {(isAuth().role === 'student') && (<img src="imgs/student.png" alt="user" className="mr-3 avatar avatar-16 img-circle img-responsive"></img>)}    
                            {(isAuth().role === 'admin') && (<img src="imgs/admin.png" alt="user" className="mr-3 avatar avatar-16 img-circle img-responsive"></img>)}    
                            {(isAuth().role === 'profesor') && (<img src="imgs/profesor.png" alt="user" className="mr-3 avatar avatar-16 img-circle img-responsive"></img>)}
                    </div>)
            }

            

            { isAuth() && (


                

                <Fragment>

                    
                    
                    <li className="nav-item ml-auto" >
                        <div>
                        <span className="nav-link " style={{cursor:'pointer', color:'#fff'}} onClick={()=> {
                                signout( () => {
                                    history.push('/')
                                })
                            }}>
                                    
                                Signout
                                
                                <svg className="bi bi-box-arrow-in-right ml-2" width="1.5em" height="1.5em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                                <path fillRule="evenodd" d="M8.146 11.354a.5.5 0 0 1 0-.708L10.793 8 8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708 0z"/>
                                <path fillRule="evenodd" d="M1 8a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9A.5.5 0 0 1 1 8z"/>
                                <path fillRule="evenodd" d="M13.5 14.5A1.5 1.5 0 0 0 15 13V3a1.5 1.5 0 0 0-1.5-1.5h-8A1.5 1.5 0 0 0 4 3v1.5a.5.5 0 0 0 1 0V3a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v10a.5.5 0 0 1-.5.5h-8A.5.5 0 0 1 5 13v-1.5a.5.5 0 0 0-1 0V13a1.5 1.5 0 0 0 1.5 1.5h8z"/>
                                </svg>
                                </span>

                        
                            
                        </div>
                        
                        
                        
                    </li>

                </Fragment>
            )}

            

            
        </ul>
        </nav>




    )
    const removeToasts = (event) => {
        console.log("HOLA");
        toast.dismiss();
    }


    return (
        
        <Fragment>
            {nav()}

            <div className="container">
                {children}
            </div>

            <div href="#" className="float" onClick={removeToasts} >
                <svg className="bi bi-archive-fill my-float" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                <path fillRule="evenodd" d="M12.643 15C13.979 15 15 13.845 15 12.5V5H1v7.5C1 13.845 2.021 15 3.357 15h9.286zM6 7a.5.5 0 0 0 0 1h4a.5.5 0 0 0 0-1H6zM.8 1a.8.8 0 0 0-.8.8V3a.8.8 0 0 0 .8.8h14.4A.8.8 0 0 0 16 3V1.8a.8.8 0 0 0-.8-.8H.8z"/>
            </svg>

            </div>
            
        </Fragment>
    )
}

export default withRouter(Layout);