import React, { Component, useState, Fragment } from 'react'
import Layout from './Layout'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import { Link } from 'react-router-dom';


const BankLoans = ({history}) => {

    const [values, setValues] = useState({
        units: [],
        unitsVisible : false,
        testnow:'Test Now!',
        nombreUnidades:[],
        loans: [],
        loansVisible : false
        

        
     });
 
    const {units , unitsVisible, testnow, nombreUnidades, loans, loansVisible} = values;
 

     React.useEffect(() => {

        axios({
            method: 'GET',
            url: `${process.env.REACT_APP_API}/loans`,
            headers:{
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }}
            ).then( function(response){
                console.log(response);
                console.log(response.data);
                setValues(values => ({...values, loans: response.data, loansVisible:true}))
                console.log("Loans object")
                console.log(loans);
                toast.success('Your bank loans have been recovered')
            })
        }, [])



    const generateBankLoansView = (event) => {

        if (!loansVisible){
            return (
                <p>We are loading your loans, please wait until this message changes</p>
            )
        }else{
            console.log("Estare bien?");
            console.log(loans);
            return (
                <div>
                <p>Nowww</p>
                
                
                    {/* {loansVisible && loans.map(function(d, idx){
                        return (<li key={idx}>{d._id}</li>)
                    })} */}



                    <table>
                        {/* <tr key={"header"}>
                            {Object.keys(state[0]).map((key) => (
                            <th>{key}</th>
                            ))}
                        </tr> */}
                        {loans.map((item) => (
                            <tr key={item._id}>
                            {Object.values(item).map((val) => (
                                <td>{val}</td>
                            ))}
                            </tr>
                        ))}
                        </table>





                </div>
                
            )
        }

        return(
            <div>
                {/* <p>{Here you can see all bank loans}</p> */}
                <p>{loansVisible && JSON.stringify(loans)}</p>

                <div>
                    {loansVisible && loans.map(function(d, idx){
                        return (<li key={idx}>{d.name}</li>)
                    })}
                    </div>

                {/* {loansVisible && 
                <ul>
                    {loans.map(l => {
                    return <li>{l._id}</li>;
                    })
                    }
                </ul>} */}
            </div>
        )
    }



    return(






        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
            <div className="row mb-4">
                <div className="col">
                    <h1>Bank Loans</h1>
                    </div>
                </div>

                <div className="row">

                    <div className="col">
      
                {generateBankLoansView()}
                </div>
                </div>
            </div>
            
        </Layout>
    )
}


export default BankLoans;