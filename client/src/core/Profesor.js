import React, { Component, useState } from 'react'
import Layout from '../core/Layout';
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import { isAuth } from '../auth/helpers';
import Asignatura from './Asignatura';

const Profesor = () => {

    const [values, setValues] = useState({
        name: '',
        roomname: '',
        buttonText:'Generar clave',
        linkProfesor: '',
        isProfesorLinkGenerated: false,
        subjectList : null,
        
    
    });

    const {name, buttonText, roomname, linkProfesor, isProfesorLinkGenerated, subjectList} = values;
    
    const handleChange = (name) => (event) => {
            setValues({ ...values, [name]: event.target.value});
    }
    
    
    const handleSubmit = event => {
        event.preventDefault();
        setValues({... values, buttonText: 'Generating'})
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/generateroom`,
            data: {
                id: isAuth()._id,
                name: isAuth().name,
                roomname
            },
            headers:{
               
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            setValues({ ...values, linkProfesor:response.data.domain,isProfesorLinkGenerated:true,  buttonText: 'Generate room' }); 
        })
        .catch(function (error) {
            setValues({...values, buttonText:'Generate Room'})
            toast.error("Something went wrong :(");
            console.log(error);
        })
    }



    React.useEffect(() => {
        setValues({ ...values});
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/user/getsubjects`,
            data: {
                id: isAuth()._id,
            },
            headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }}).then(function (response) {
                setValues(values => ({...values, subjectList : response.data.subjectList}))
                console.info("LO QUE RECIBI");
                console.log(response.data.subjectList);
                console.log(subjectList);
                //toast.success("Bieen");
            }).catch(function (error) {
                toast.error(error);
                console.log(error);
            })
    }, [])


    const claveProfesor = () => (
        <a href={values.linkProfesor}>Teacher Room</a>
    )

    const asignatura = () => (
        <form>
            <div className="form-group">
            <label className="text-muted">Room name</label>
                <input onChange={handleChange('roomname')} type="text" className="form-control" placeholder="Arquitectura de Computadores">

                </input>

                <div>
                    <button className="btn btn-primary" onClick={(handleSubmit)}>{buttonText}</button>
                </div>
            
            </div>
        </form>
    )



    return (
        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
                <div className="row mb-4">
                    <div className="col">
                        <h1>Profesor : {isAuth().name}</h1>
                    </div>
                </div>
                <div className="row">
                    <div className="col">
                        {/* {asignatura()} */}
                        {subjectList && 
                            subjectList.map((item,key) => {return <Asignatura key={key} index={key} data={item}></Asignatura>})
                            
                        }
                      
                        
                    </div>
                </div>



                {isProfesorLinkGenerated &&  
                    <div className="row">
                        <div className="col-sm-8 offset-md-1">
                        {claveProfesor()}
                        </div>

                        

                        
                    </div>
                }
                
            </div>

            

            
        </Layout>
    );
    
    
}




export default Profesor;