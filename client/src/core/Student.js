import React, { Component, useState, Fragment } from 'react'
import Layout from '../core/Layout'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
// import { isAuth } from '../auth/helpers';
// import Play from '../quiz/Play';
import { Link } from 'react-router-dom';


const Student = ({history}) => {

    const [values, setValues] = useState({
        units: [],
        unitsVisible : false,
        testnow:'Test Now!',
        nombreUnidades:[]
        

        
     });
 
    const {units , unitsVisible, testnow, nombreUnidades} = values;
 

     React.useEffect(() => {
         
         axios({
             method: 'POST',
             url: `${process.env.REACT_APP_API}/units/list`,
             headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }}).then(function(response ){
                console.log("[/units/list] Received:", response.data.listUnidadesName);
                setValues(values => ({...values, nombreUnidades: response.data.listUnidadesName}))
            })
        }, [])






    const handleTestUnit = (item) =>  (event) => {
        history.push(   {pathname: '/testunits',
                        state:item})
        console.log("EL boton ve:", item)

    }


    const handleTestExam = (name) => (event) => {
        toast.success('A por el test', name)
    }
    const generateStudentView = (event) => {
        return (
            <div>
                <div className="card mb-5 ">
                <div className="card-header text-center h3">Test por Unidades</div>
                <div className="card-body">
                    <div className="row">
                    {
                        <ul>
                            <p className="h4 ml-4 mb-4">Modelo A</p>

                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u1">Unidad 1 2019_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u2">Unidad 2 2019_1C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u3">Unidad 3 2019_1C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u4">Unidad 4 2019_1C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u5">Unidad 5 2019_1C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u6">Unidad 6 2019_1C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u7">Unidad 7 2019_1C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u8">Unidad 8 2019_1C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u9">Unidad 9 2019_1C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u10">Unidad 10 2019_1C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u11">Unidad 11 2019_1C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u1">Unidad 1 2019_2C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u2">Unidad 2 2019_2C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u3">Unidad 3 2019_2C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u4">Unidad 4 2019_2C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u5">Unidad 5 2019_2C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u6">Unidad 6 2019_2C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u7">Unidad 7 2019_2C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u8">Unidad 8 2019_2C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u9">Unidad 9 2019_2C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u10">Unidad 10 2019_2C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_A_u11">Unidad 11 2019_2C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u1">Unidad 1 2019_3C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u2">Unidad 2 2019_3C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u3">Unidad 3 2019_3C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u4">Unidad 4 2019_3C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u5">Unidad 5 2019_3C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u6">Unidad 6 2019_3C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u7">Unidad 7 2019_3C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u8">Unidad 8 2019_3C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u9">Unidad 9 2019_3C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u10">Unidad 10 2019_3C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_A_u11">Unidad 11 2019_3C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>



                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u1">Unidad 1 2018_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u2">Unidad 2 2018_1C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u3">Unidad 3 2018_1C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u4">Unidad 4 2018_1C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u5">Unidad 5 2018_1C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u6">Unidad 6 2018_1C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u7">Unidad 7 2018_1C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u8">Unidad 8 2018_1C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u9">Unidad 9 2018_1C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u10">Unidad 10 2018_1C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u11">Unidad 11 2018_1C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u1">Unidad 1 2018_2C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u2">Unidad 2 2018_2C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u3">Unidad 3 2018_2C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u4">Unidad 4 2018_2C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u5">Unidad 5 2018_2C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u6">Unidad 6 2018_2C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u7">Unidad 7 2018_2C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u8">Unidad 8 2018_2C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u9">Unidad 9 2018_2C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u10">Unidad 10 2018_2C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_A_u11">Unidad 11 2018_2C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u1">Unidad 1 2018_3C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u2">Unidad 2 2018_3C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u3">Unidad 3 2018_3C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u4">Unidad 4 2018_3C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u5">Unidad 5 2018_3C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u6">Unidad 6 2018_3C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u7">Unidad 7 2018_3C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u8">Unidad 8 2018_3C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u9">Unidad 9 2018_3C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u10">Unidad 10 2018_3C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_A_u11">Unidad 11 2018_3C_A_u11 Carta de Navegación</Link></li>
                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u1">Unidad 1 2018_4C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u2">Unidad 2 2018_4C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u3">Unidad 3 2018_4C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u4">Unidad 4 2018_4C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u5">Unidad 5 2018_4C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u6">Unidad 6 2018_4C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u7">Unidad 7 2018_4C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u8">Unidad 8 2018_4C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u9">Unidad 9 2018_4C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u10">Unidad 10 2018_4C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_A_u11">Unidad 11 2018_4C_A_u11 Carta de Navegación</Link></li>




                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>



                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u1">Unidad 1 2017_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u2">Unidad 2 2017_1C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u3">Unidad 3 2017_1C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u4">Unidad 4 2017_1C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u5">Unidad 5 2017_1C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u6">Unidad 6 2017_1C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u7">Unidad 7 2017_1C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u8">Unidad 8 2017_1C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u9">Unidad 9 2017_1C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u10">Unidad 10 2017_1C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u11">Unidad 11 2017_1C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u1">Unidad 1 2017_2C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u2">Unidad 2 2017_2C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u3">Unidad 3 2017_2C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u4">Unidad 4 2017_2C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u5">Unidad 5 2017_2C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u6">Unidad 6 2017_2C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u7">Unidad 7 2017_2C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u8">Unidad 8 2017_2C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u9">Unidad 9 2017_2C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u10">Unidad 10 2017_2C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_A_u11">Unidad 11 2017_2C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u1">Unidad 1 2017_3C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u2">Unidad 2 2017_3C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u3">Unidad 3 2017_3C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u4">Unidad 4 2017_3C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u5">Unidad 5 2017_3C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u6">Unidad 6 2017_3C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u7">Unidad 7 2017_3C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u8">Unidad 8 2017_3C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u9">Unidad 9 2017_3C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u10">Unidad 10 2017_3C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_A_u11">Unidad 11 2017_3C_A_u11 Carta de Navegación</Link></li>





                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>



                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u1">Unidad 1 2016_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u2">Unidad 2 2016_1C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u3">Unidad 3 2016_1C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u4">Unidad 4 2016_1C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u5">Unidad 5 2016_1C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u6">Unidad 6 2016_1C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u7">Unidad 7 2016_1C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u8">Unidad 8 2016_1C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u9">Unidad 9 2016_1C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u10">Unidad 10 2016_1C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u11">Unidad 11 2016_1C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u1">Unidad 1 2016_2C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u2">Unidad 2 2016_2C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u3">Unidad 3 2016_2C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u4">Unidad 4 2016_2C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u5">Unidad 5 2016_2C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u6">Unidad 6 2016_2C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u7">Unidad 7 2016_2C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u8">Unidad 8 2016_2C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u9">Unidad 9 2016_2C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u10">Unidad 10 2016_2C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_A_u11">Unidad 11 2016_2C_A_u11 Carta de Navegación</Link></li>


                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u1">Unidad 1 2016_3C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u2">Unidad 2 2016_3C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u3">Unidad 3 2016_3C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u4">Unidad 4 2016_3C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u5">Unidad 5 2016_3C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u6">Unidad 6 2016_3C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u7">Unidad 7 2016_3C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u8">Unidad 8 2016_3C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u9">Unidad 9 2016_3C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u10">Unidad 10 2016_3C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_A_u11">Unidad 11 2016_3C_A_u11 Carta de Navegación</Link></li>










                            <br/>
                            <br/>
                            <br/>



                            <p className="h4 ml-4 mt-4 mb-4">Modelo B</p>
                            
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u1">Unidad 1 2019_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u2">Unidad 2 2019_1C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u3">Unidad 3 2019_1C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u4">Unidad 4 2019_1C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u5">Unidad 5 2019_1C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u6">Unidad 6 2019_1C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u7">Unidad 7 2019_1C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u8">Unidad 8 2019_1C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u9">Unidad 9 2019_1C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u10">Unidad 10 2019_1C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_B_u11">Unidad 11 2019_1C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u1">Unidad 1 2019_2C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u2">Unidad 2 2019_2C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u3">Unidad 3 2019_2C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u4">Unidad 4 2019_2C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u5">Unidad 5 2019_2C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u6">Unidad 6 2019_2C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u7">Unidad 7 2019_2C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u8">Unidad 8 2019_2C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u9">Unidad 9 2019_2C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u10">Unidad 10 2019_2C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_2C_B_u11">Unidad 11 2019_2C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u1">Unidad 1 2019_3C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u2">Unidad 2 2019_3C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u3">Unidad 3 2019_3C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u4">Unidad 4 2019_3C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u5">Unidad 5 2019_3C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u6">Unidad 6 2019_3C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u7">Unidad 7 2019_3C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u8">Unidad 8 2019_3C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u9">Unidad 9 2019_3C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u10">Unidad 10 2019_3C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_3C_B_u11">Unidad 11 2019_3C_B_u11 Carta de Navegación</Link></li>





                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>




                            <li><Link className="play-button" to="/play/quiz/2018_1C_A_u1">Unidad 1 2018_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u2">Unidad 2 2018_1C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u3">Unidad 3 2018_1C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u4">Unidad 4 2018_1C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u5">Unidad 5 2018_1C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u6">Unidad 6 2018_1C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u7">Unidad 7 2018_1C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u8">Unidad 8 2018_1C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u9">Unidad 9 2018_1C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u10">Unidad 10 2018_1C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_1C_B_u11">Unidad 11 2018_1C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u1">Unidad 1 2018_2C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u2">Unidad 2 2018_2C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u3">Unidad 3 2018_2C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u4">Unidad 4 2018_2C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u5">Unidad 5 2018_2C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u6">Unidad 6 2018_2C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u7">Unidad 7 2018_2C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u8">Unidad 8 2018_2C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u9">Unidad 9 2018_2C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u10">Unidad 10 2018_2C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_2C_B_u11">Unidad 11 2018_2C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u1">Unidad 1 2018_3C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u2">Unidad 2 2018_3C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u3">Unidad 3 2018_3C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u4">Unidad 4 2018_3C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u5">Unidad 5 2018_3C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u6">Unidad 6 2018_3C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u7">Unidad 7 2018_3C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u8">Unidad 8 2018_3C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u9">Unidad 9 2018_3C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u10">Unidad 10 2018_3C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_3C_B_u11">Unidad 11 2018_3C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u1">Unidad 1 2018_4C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u2">Unidad 2 2018_4C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u3">Unidad 3 2018_4C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u4">Unidad 4 2018_4C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u5">Unidad 5 2018_4C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u6">Unidad 6 2018_4C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u7">Unidad 7 2018_4C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u8">Unidad 8 2018_4C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u9">Unidad 9 2018_4C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u10">Unidad 10 2018_4C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2018_4C_B_u11">Unidad 11 2018_4C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>



                            <li><Link className="play-button" to="/play/quiz/2017_1C_A_u1">Unidad 1 2017_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u2">Unidad 2 2017_1C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u3">Unidad 3 2017_1C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u4">Unidad 4 2017_1C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u5">Unidad 5 2017_1C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u6">Unidad 6 2017_1C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u7">Unidad 7 2017_1C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u8">Unidad 8 2017_1C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u9">Unidad 9 2017_1C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u10">Unidad 10 2017_1C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_1C_B_u11">Unidad 11 2017_1C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u1">Unidad 1 2017_2C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u2">Unidad 2 2017_2C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u3">Unidad 3 2017_2C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u4">Unidad 4 2017_2C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u5">Unidad 5 2017_2C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u6">Unidad 6 2017_2C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u7">Unidad 7 2017_2C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u8">Unidad 8 2017_2C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u9">Unidad 9 2017_2C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u10">Unidad 10 2017_2C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_2C_B_u11">Unidad 11 2017_2C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u1">Unidad 1 2017_3C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u2">Unidad 2 2017_3C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u3">Unidad 3 2017_3C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u4">Unidad 4 2017_3C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u5">Unidad 5 2017_3C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u6">Unidad 6 2017_3C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u7">Unidad 7 2017_3C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u8">Unidad 8 2017_3C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u9">Unidad 9 2017_3C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u10">Unidad 10 2017_3C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2017_3C_B_u11">Unidad 11 2017_3C_B_u11 Carta de Navegación</Link></li>



                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>
                            <br/>



                            <li><Link className="play-button" to="/play/quiz/2016_1C_A_u1">Unidad 1 2016_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u2">Unidad 2 2016_1C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u3">Unidad 3 2016_1C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u4">Unidad 4 2016_1C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u5">Unidad 5 2016_1C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u6">Unidad 6 2016_1C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u7">Unidad 7 2016_1C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u8">Unidad 8 2016_1C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u9">Unidad 9 2016_1C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u10">Unidad 10 2016_1C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_1C_B_u11">Unidad 11 2016_1C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u1">Unidad 1 2016_2C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u2">Unidad 2 2016_2C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u3">Unidad 3 2016_2C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u4">Unidad 4 2016_2C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u5">Unidad 5 2016_2C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u6">Unidad 6 2016_2C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u7">Unidad 7 2016_2C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u8">Unidad 8 2016_2C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u9">Unidad 9 2016_2C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u10">Unidad 10 2016_2C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_2C_B_u11">Unidad 11 2016_2C_B_u11 Carta de Navegación</Link></li>

                            <br/>
                            <br/>

                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u1">Unidad 1 2016_3C_B_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u2">Unidad 2 2016_3C_B_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u3">Unidad 3 2016_3C_B_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u4">Unidad 4 2016_3C_B_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u5">Unidad 5 2016_3C_B_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u6">Unidad 6 2016_3C_B_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u7">Unidad 7 2016_3C_B_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u8">Unidad 8 2016_3C_B_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u9">Unidad 9 2016_3C_B_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u10">Unidad 10 2016_3C_B_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2016_3C_B_u11">Unidad 11 2016_3C_B_u11 Carta de Navegación</Link></li>








                            <br/>
                            <br/>
                            <br/>




                        </ul>
                     }

                    </div>

                </div>
                </div>



                <div className="card mb-5 ">
                <div className="card-header text-center h3">Test por Exámenes Completos</div>
                <div className="card-body">


                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_1C_A">Examen 2019 Modelo A Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_2C_A">Examen 2019 Modelo A Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_3C_A">Examen 2019 Modelo A Convocatoria 3</Link>
                            </div>
                        </div>

                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_1C_A">Examen 2018 Modelo A Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_2C_A">Examen 2018 Modelo A Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_3C_A">Examen 2018 Modelo A Convocatoria 3</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_4C_A">Examen 2018 Modelo A Convocatoria 4</Link>
                            </div>
                        </div>

                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_1C_A">Examen 2017 Modelo A Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_2C_A">Examen 2017 Modelo A Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_3C_A">Examen 2017 Modelo A Convocatoria 3</Link>
                            </div>
                        </div>

                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_1C_A">Examen 2016 Modelo A Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_2C_A">Examen 2016 Modelo A Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_3C_A">Examen 2016 Modelo A Convocatoria 3</Link>
                            </div>
                        </div>

                        
                        
                        
                        <br/>
                        <br/>
                        <br/>

                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_1C_B">Examen 2019 Modelo B Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_2C_B">Examen 2019 Modelo B Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2019_3C_B">Examen 2019 Modelo B Convocatoria 3</Link>
                            </div>
                        </div>


                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_1C_B">Examen 2018 Modelo B Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_2C_B">Examen 2018 Modelo B Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_3C_B">Examen 2018 Modelo B Convocatoria 3</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2018_4C_B">Examen 2018 Modelo B Convocatoria 4</Link>
                            </div>
                        </div>


                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_1C_B">Examen 2017 Modelo B Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_2C_B">Examen 2017 Modelo B Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2017_3C_B">Examen 2017 Modelo B Convocatoria 3</Link>
                            </div>
                        </div>

                        <div className="row">
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_1C_B">Examen 2016 Modelo B Convocatoria 1</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_2C_B">Examen 2016 Modelo B Convocatoria 2</Link>
                            </div>
                            <div class="col-sm">
                                <Link className="play-button" to="/play/quiz/2016_3C_B">Examen 2016 Modelo B Convocatoria 3</Link>
                            </div>
                        </div>






                </div>
                </div>



                <div className="card mb-5 ">
                <div className="card-header text-center h3">Triggered DB</div>
                <div className="card-body">
                    <div className="row">
                    <div className="col mb-4"> 

                    <p className="h4 ml-4 mb-4">Modelo A</p>
                            

                            {nombreUnidades && nombreUnidades.map((unidad, indice) => 
                                {if(unidad.indexOf('u') !== -1 && unidad.indexOf('A') !== -1 ){
                                    return <Link  key={indice} className="play-button" to={"/play/quiz/"+unidad} >{unidad}</Link>
                                }
                                })}
                            

                            {/* <li><Link className="play-button" to="/play/quiz/2019_1C_A_u1">Unidad 1 2019_1C_A_u1 Nomenclatura Náutica </Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u2">Unidad 2 2019_1C_A_u2 Elementos de amarre y fondeo</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u3">Unidad 3 2019_1C_A_u3 Seguridad en la mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u4">Unidad 4 2019_1C_A_u4 Legislación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u5">Unidad 5 2019_1C_A_u5 Balizamiento</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u6">Unidad 6 2019_1C_A_u6 Reglamento de Abordajes</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u7">Unidad 7 2019_1C_A_u7 Maniobra y Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u8">Unidad 8 2019_1C_A_u8 Emergencias en la Mar</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u9">Unidad 9 2019_1C_A_u9 Meteorología</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u10">Unidad 10 2019_1C_A_u10 Teoría de Navegación</Link></li>
                            <li><Link className="play-button" to="/play/quiz/2019_1C_A_u11">Unidad 11 2019_1C_A_u11 Carta de Navegación</Link></li> */}

                            <br/>
                            <br/>
                            <br/>

                            <p className="h4 ml-4 mt-4 mb-4">Modelo B</p>
                            
                            {/* {nombreUnidades && nombreUnidades.map((unidad, indice) => 
                                {if(unidad.indexOf('u') !== -1 && unidad.indexOf('B') !== -1 ){
                                    return <Link  key={indice} className="play-button" to={"/play/quiz/"+unidad} >{unidad}</Link>
                                }
                                
                                
                                })} */}

                    
                    </div>
                </div>
                </div>
                </div>





                




                     



        </div>
        )
    }



    return(






        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
            <div className="row mb-4">
                <div className="col">
                    <h1>Student Page</h1>
                    </div>
                </div>

                <div className="row">

                    <div className="col">
      
                {generateStudentView()}
                </div>
                </div>
            </div>
            
        </Layout>
    )
}


export default Student;