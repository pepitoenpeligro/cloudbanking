import React, {useState} from 'react'
import {Link, Redirect} from 'react-router-dom'
import Layout from '../core/Layout'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';


const Signup = ({history}) => {
    const [values, setValues] = useState({
        name:'',
        email:'',
        password:'',
        buttonText:'Register'

    });

    const {name, email, password, buttonText} = values;

    const handleChange = (name) => (event) => {
        setValues({ ...values, [name]: event.target.value});
    }

    const handleSubmit = event => {
        event.preventDefault()
        setValues({... values, buttonText: 'Submitting'})
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/signup`,
            data: {
                name: name,
                email: email,
                password: password
            },
            headers:{
                'Content-Type': 'application/json'
            }
        }).then(function (response) {
            setValues({...values, name:'', email:'', password:'', buttonText:'Register'})
            toast.success("You are now registered");
            history.push('/signin');
            
        })
        .catch(function (error) {
            setValues({...values, buttonText:'Register'})
            
            toast.error("Something went wrong :(");
            console.log(error);
        }).finally(function (e){
            setValues({...values, name:'', email: '', password:''})
        })
    }
    



    const signupForm = () => (
        <div className="col-md-6 offset-md-3">
        <h1 className="p5-5 text-center">Signup</h1>
        <form>
            <div className="form-group">
                <div className="mt-3">
                <label className="text-muted">Name</label>
                <input onChange={handleChange('name')} type="text" className="form-control" placeholder="Name LastName"></input>
                </div>
                <div className="mt-3">
                <label className="text-muted">Email</label>
                <input onChange={handleChange('email')} type="email" className="form-control" placeholder="newuser@domain"></input>
                </div>
            
                <div className="mt-3">
                <label className="text-muted">Password</label>
                <input onChange={handleChange('password')} type="password"  placeholder="password" className="form-control"></input>
                </div>

                <div className="text-center mt-2">
                <small>You'll be registered as student</small>
                </div> 
                
                <div className="text-center mt-4">
                    <button type="submit" className="btn btn-primary" onClick={(handleSubmit)}>{buttonText}</button>
                    
                </div>
                
            
            </div>
        </form>
        </div>
    )
    
    
    return (
        <Layout>
            <ToastContainer/>
            {signupForm()}
        </Layout>
    )
}

export default Signup;