import React, { Component, useState, Fragment } from 'react'
import Layout from './Layout'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import { Link } from 'react-router-dom';


const BankAccounts = ({history}) => {

    const [values, setValues] = useState({
        units: [],
        unitsVisible : false,
        testnow:'Test Now!',
        nombreUnidades:[]
        

        
     });
 
    const {units , unitsVisible, testnow, nombreUnidades} = values;
 

     React.useEffect(() => {
         
        //  axios({
        //      method: 'POST',
        //      url: `${process.env.REACT_APP_API}/units/list`,
        //      headers:{
        //         'Content-Type': 'application/json',
        //         'Accept':'application/json'
        //     }}).then(function(response ){
        //         console.log("[/units/list] Received:", response.data.listUnidadesName);
        //         setValues(values => ({...values, nombreUnidades: response.data.listUnidadesName}))
        //     })
        }, [])






    const handleTestUnit = (item) =>  (event) => {
        history.push(   {pathname: '/testunits',
                        state:item})
        console.log("EL boton ve:", item)

    }


    const handleTestExam = (name) => (event) => {
        toast.success('A por el test', name)
    }

    const generateBankAccountView = (event) => {
        return(
            <div>
                <p>Here you can see all bank accounts</p>
            </div>
        )
    }




    return(






        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
            <div className="row mb-4">
                <div className="col">
                    <h1>Bank Accounts</h1>
                    </div>
                </div>

                <div className="row">

                    <div className="col">
      
                {generateBankAccountView()}
                </div>
                </div>
            </div>
            
        </Layout>
    )
}


export default BankAccounts;