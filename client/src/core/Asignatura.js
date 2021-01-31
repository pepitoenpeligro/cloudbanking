import React, { Component, useState, Fragment } from 'react'
import Layout from '../core/Layout';
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import { isAuth } from '../auth/helpers';
import StudentElement from './StudentElement';
import JitsiElement from './JitsiElement';




const Item = (props) => {

    
    

    const [values, setValues] = useState({
        isGeneratingRoom:false,
        buttonText: '1. Generate room for proffesor',
        buttonTextStudent: '2. Generate for students',
        domain: '',
        studentsList: [],
        subjectListActive: '',
        isBtnGenerateRoomActive: true,
        isBtnGenerateForStudentActive: true,
        isJitsiActive: false,
        key:props.key



  

    });

    

    const {key, domain2, options,isGeneratingRoom, buttonText,buttonTextStudent, domain,studentsList,isJitsiActive,  subjectListActive, isBtnGenerateRoomActive, isBtnGenerateForStudentActive } = values;

    const handleGenrarAula = event => {
        //console.log("Aula generada", props.data._id);
        event.preventDefault();
        setValues({... values, buttonText: 'Generating'})
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/generateroom`,
            data: {
                id: isAuth()._id,
                name: isAuth().name,
                roomname: props.data.title
            },
            headers:{
               
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            toast.success(`Room generated for ${props.data.title}`);
            setValues({ ...values, isGeneratingRoom:true, domain:response.data.domain,  buttonText: 'Generated', isBtnGenerateRoomActive:false }); 
        })
        .catch(function (error) {
            setValues({...values, buttonText:'Generate room'})
            toast.error("Something went wrong :(");
            console.log(error);
        })
    }

    const handleGenerateStudents = event => {
        event.preventDefault();
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/getstudents`,
            data: {
                idSubject: props.data._id
            },
            headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
           console.log("recibido", response);
           
           setValues({...values, studentsList: response.data.users, subjectListActive: response.data.subjectId, isBtnGenerateForStudentActive:false, buttonTextStudent:'Generated'});
           toast.success("All students obtained");
           console.log("[/getstudents] All students obtained", response.data.users);
        })
        .catch(function (error) {
            toast.error(`No students in this subject`);
            console.log(`[/getstudents]No students in this subject`, error);
        })

    }

    const tableUsers = event => {
        return (

            <div className="row mt-5 mb-4 ml-2 mr-2">
                <div className="col mt-1">
                <h3>Students's Manager</h3>
                
                    <div className="table-responsive-xl">
                        <table className="table table-striped table-sm">
                            <thead>
                            <tr>
                                <th scope="col">#</th>
                                <th scope="col">Name</th>
                                <th scope="col w-25">Email</th>
                                <th scope="col">Jitsi Link</th>
                                <th scope="col">Action 1</th>
                                <th scope="col">Action 2</th>
                            </tr>
                            </thead>

                            <tbody>
                                {values.studentsList.map((item,index) => {return <StudentElement key={index} indice={index} data={item} nameSubject={props.data.title} idSubject={props.data._id}></StudentElement>})}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        )
    }

    const set = (event) => {

    }

    const setupJitsiEmbed = (event) => {
        return (

            <JitsiElement key={key} index={props.index} token={domain} name={isAuth().name}></JitsiElement>
 

        )
    }

    const componetProffesorAccess= (event) => {
        return (
            
            <div className="row ml-2 mt-4 mb-5">
                <div className="col">
                <h3>Room Generated</h3>
                    <div>
                        <svg className="bi bi-arrow-bar-right mr-2" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                        <path fillRule="evenodd" d="M10.146 4.646a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L12.793 8l-2.647-2.646a.5.5 0 0 1 0-.708z"/>
                        <path fillRule="evenodd" d="M6 8a.5.5 0 0 1 .5-.5H13a.5.5 0 0 1 0 1H6.5A.5.5 0 0 1 6 8zm-2.5 6a.5.5 0 0 1-.5-.5v-11a.5.5 0 0 1 1 0v11a.5.5 0 0 1-.5.5z"/>
                        </svg>
                        <a href={domain} >Proffesor's access</a>
                    </div>
                    <div>
                        <svg className="bi bi-link mr-2" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                        <path d="M6.354 5.5H4a3 3 0 0 0 0 6h3a3 3 0 0 0 2.83-4H9c-.086 0-.17.01-.25.031A2 2 0 0 1 7 10.5H4a2 2 0 1 1 0-4h1.535c.218-.376.495-.714.82-1z"/>
                        <path d="M6.764 6.5H7c.364 0 .706.097 1 .268A1.99 1.99 0 0 1 9 6.5h.236A3.004 3.004 0 0 0 8 5.67a3 3 0 0 0-1.236.83z"/>
                        <path d="M9 5.5a3 3 0 0 0-2.83 4h1.098A2 2 0 0 1 9 6.5h3a2 2 0 1 1 0 4h-1.535a4.02 4.02 0 0 1-.82 1H12a3 3 0 1 0 0-6H9z"/>
                        <path d="M8 11.33a3.01 3.01 0 0 0 1.236-.83H9a1.99 1.99 0 0 1-1-.268 1.99 1.99 0 0 1-1 .268h-.236c.332.371.756.66 1.236.83z"/>
                        </svg>
                        <small>{domain}</small>
                    </div>
                    
                    
                    
                    <div>

                    </div>
                </div>
                
            </div>
        )
    }

    return(
        <Fragment>


        <div className="card mb-5 ">
            <div className="card-header text-center h3">{props.data.title}</div>
            <div className="card-body">
                <div className="row">

                    <div className="col mb-4"> 
                        <button className="btn btn-primary btn-block" disabled={isBtnGenerateRoomActive?false:true} onClick={handleGenrarAula}>{buttonText}</button>
                    </div>

                    <div className="col mb-4"> 
                        <button className="btn btn-primary btn-block" disabled={isBtnGenerateForStudentActive?false:true} onClick={handleGenerateStudents}>{buttonTextStudent}</button>  
                    </div>

                </div>

                

                

                
                {isGeneratingRoom && componetProffesorAccess()}
                
    
                {studentsList && subjectListActive===props.data._id && tableUsers()}


                { (<div className="row mt-3 ml-1">

                            <div className="container"> 
                            <div className={`${"jitsi-container-"}${props.index}`}>
                                
                                <p>Embbed Jitsi</p>
                                    <button className="btn btn-primary btn-sm" onClick={() => {setValues({...values, isJitsiActive: !isJitsiActive})}} >Embed Jitsi</button>
                                    {isJitsiActive &&  setupJitsiEmbed()}
                                    </div>
                            </div>
                        </div>)}
            </div>
        </div>
        </Fragment>
        
    )
}

class Asignatura extends React.Component{

    constructor(props){
        super(props);
    }

    render(){
        return(
            <div>

                <ToastContainer/>
                <Item index={this.props.index} data={this.props.data}></Item>
            </div>
        )
    }
}



export default Asignatura;