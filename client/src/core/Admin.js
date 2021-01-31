import React, { Component, useState } from 'react'
import Layout from '../core/Layout';
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import { isAuth } from '../auth/helpers';

const Admin = () => {

    const [values, setValues] = useState({
       isActiveAddSubject: false,
       buttonTextFormAddSubject: 'Save',
       subjectname: '',
       isActivaAddSubjectProfesor: false,
       isActiveAddStudentinSubject:false,
       buttonTextFormAddSubjectProfesor: 'Save',
       buttonTextFormAddStudentSubject: 'Save',
       email: ''
       
    });

    const { isActiveAddSubject, buttonTextFormAddSubject, subjectname, isActivaAddSubjectProfesor, isActiveAddStudentinSubject, buttonTextFormAddStudentSubject,  buttonTextFormAddSubjectProfesor, email} = values;

    const handleChangeFormAddSubject = (name) => (event) => {
        setValues({ ...values, [name]: event.target.value});
        console.log(subjectname);
    }

    const handleChange = (name) => (event) => {
        setValues({ ...values, [name]: event.target.value});
    }

    const handleButtonAddStudentinSubject= event =>{
        setValues({...values, isActiveAddStudentinSubject: !isActiveAddStudentinSubject})
    }

    const handleAddSubject = event => {
        setValues({...values, isActiveAddSubject: !isActiveAddSubject})
    }

    const handleAddSubjectProfesor = event => {
        setValues({...values, isActivaAddSubjectProfesor: !isActivaAddSubjectProfesor})
    }


    const handleSubmitAddSubject = event => {
        event.preventDefault();
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/addsubject`,
            data: {
                subjectname
            },
            headers:{
               
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            setValues({...values, isActiveAddSubject:false})
            toast.success(`${subjectname} added`);
            console.log("Insertado");
        })
        .catch(function (error) {
            setValues({...values, buttonText:'Generate Room'})
            toast.error("Something went wrong :(");
            console.log(error);
        })

    }


    const handleChangeAddSubjectProfesor = (name) => (event) => {
        setValues({ ...values, [name]: event.target.value});
    }

    


    const handleSubmitAddSubjectProfesor = event => {
        event.preventDefault();
        
        setValues({ ...values});
        console.log("enviando", values, subjectname)
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/addsubjectprofesor`,
            data: {
                id: isAuth()._id,
                email,
                subjectname
            },
            headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            toast.success(`${email} added in ${subjectname}`);
            setValues({ ...values}); 
            console.log(`${email} added in ${subjectname}`)
        })
        .catch(function (error) {
            toast.error(`Error adding ${email} in ${subjectname}`);
            console.log(`Error adding ${email} in ${subjectname}`);
        })
    

    }



    const handleSubmitAddStudentSubject = event => {
        event.preventDefault();
        
        setValues({ ...values});
        console.log("VOY A ENVIAR", values)
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/addstudentinsubject`,
            data: {
                email,
                subjectname
            },
            headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            toast.success(`Student ${email} added in ${subjectname}`);
            setValues({ ...values}); 
            console.log(`Student ${email} added in ${subjectname}`, response)
        })
        .catch(function (error) {
            toast.error(`Error adding ${email} in ${subjectname}`);
            console.log(`Error adding ${email} in ${subjectname}`);
        })

        

    }

    const formAddSubject = event => {
        return (
            <form>
                <div className="form-group">
                <label className="text-muted">Subject Name</label>
                    <input onChange={handleChangeFormAddSubject('subjectname')} type="text" className="form-control" placeholder="Arquitectura de Computadores">
                    </input>
                    <div>
                        <button className="btn btn-primary" onClick={(handleSubmitAddSubject)}>{buttonTextFormAddSubject}</button>
                    </div>
                
                </div>
            </form>
        )
    }

    const formAddSubjectProfesor = event =>{
        return (
            <form>
                <div className="form-group">
                <label className="text-muted">Subject Name</label>
                    <input onChange={handleChangeAddSubjectProfesor('subjectname')} type="text" className="form-control" placeholder="Arquitectura de Computadores">
                    </input>
                    <label className="text-muted">Professor's Email</label>
                    <input onChange={handleChangeAddSubjectProfesor('email')} type="text" className="form-control" placeholder="profesor@ugr.es">
                    </input>
                    <div>
                        <button onClick={handleSubmitAddSubjectProfesor} className="btn btn-primary" >{buttonTextFormAddSubjectProfesor}</button>
                    </div>
                
                </div>
            </form>



        )
    }


    const formAddStudentSubject = event => {
        return (


            <form>
            <div className="form-group">
                <label className="text-muted">Subject name</label>
                <input  onChange={handleChange('subjectname')} type="text" className="form-control" placeholder="Arquitectura de Computadores" >
                
                </input>
                <label className="text-muted">Student email</label>
                <input onChange={handleChange('email')} type="text" className="form-control" placeholder="student@ugr.es">
                </input>
                <div>
                    <button onClick={handleSubmitAddStudentSubject} className="btn btn-primary" >{buttonTextFormAddStudentSubject}</button>
                    
                </div>
            
            </div>
        </form>
            
        )
    }

    return (
        <Layout>
            <ToastContainer/>
            <div className=" mt-4 mb-4">
                <h1>Admin page</h1>
                <div>
                    <div >
                    <h2>Subject</h2>
                    <button className="btn btn-primary" onClick={handleAddSubject}>Add Subject</button>
                    {isActiveAddSubject && formAddSubject() }
                    </div>

                    <div><br/></div>

                    <div >
                    <h2>Subject-Profesor</h2>
                    <button className="btn btn-primary" onClick={handleAddSubjectProfesor}>Add Profesor to Subject</button>
                    {isActivaAddSubjectProfesor && formAddSubjectProfesor()}
                    </div>

                    <div><br/></div>

                    <div >
                    <h2>Add Student in Subject</h2>
                    <button className="btn btn-primary" onClick={handleButtonAddStudentinSubject}>Add Student to Subject</button>
                    {isActiveAddStudentinSubject && formAddStudentSubject()}
                    </div>
                    


                </div>
            </div>
            
            
            
        </Layout>
    );
    
    
}


export default Admin;