import React, { Component, useState, Fragment } from 'react'
import { useLocation } from "react-router-dom";
import Layout from './Layout';
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';



import { isAuth } from '../auth/helpers';

import {Form } from 'react-bootstrap'








const Units = (props, {history}) => {
    // const estadoAnterior = useLocation();
    
    

    // const [values, setValues] = useState({
    //     mapaPreguntas: [{pregunta:'Esta', respuestas:['','','',''], correcta:-1}, {pregunta:'Otra', respuestas:['','','',''], correcta:-1}]

    // });

    

    // const {mapaPreguntas} = values;

    // React.useEffect(() => {
    //     console.log("Arranca Units")
    //     console.log("Units ve:",estadoAnterior.state);
    //     setValues({...setValues, mapaPreguntas: estadoAnterior.state.questions})
        
    //  },[mapaPreguntas]);




    // return(
        
    //             <Layout>
    //             <ToastContainer/>
    //             <div className="container mt-4 mb-4">
    //             <div className="row mb-4">
    //             <div className="col">
    //                 <h1>Testing</h1>
    //             </div>
    //             </div>
    //             <div className="row">

    //             {
    //             mapaPreguntas.map((item, index) => {
    //                 console.log("Veo en el map: ", item)
    //             return (
                
    //                 <Unit key={index} data={item}></Unit>
    //             // <h3>{item.question}</h3>

    //             )
                
    //             })
    //             }
                
    //             {/* <Unit index={this.props.index} data={this.props.data}></Unit> */}
    //             </div>
    //             </div>
    //             </Layout>
    // )

    


}



export default Units;