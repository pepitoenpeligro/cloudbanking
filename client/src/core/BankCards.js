import React, {useState} from 'react'
import Layout from './Layout'
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';

import Lottie from "react-lottie";


const BankCards = ({history}) => {

    const [values, setValues] = useState({
        units: [],
        unitsVisible : false,
        testnow:'Test Now!',
        nombreUnidades:[]
     });
 
    const {} = values;


    const animationOptions = {
        loop: true,
        autoplay: true,
        path: "https://assets3.lottiefiles.com/packages/lf20_y0hSoJ.json"
        // height: 100,
        // rendererSettings: {
        //   preserveAspectRatio: "xMidYMid slice",
        // },
      };
 

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






    // const handleTestUnit = (item) =>  (event) => {
    //     history.push({pathname: '/testunits',state:item})
    //     console.log("EL boton ve:", item)
    // }


    const generateBankCardsView = (event) => {
        return(
            <div>
                <p>Here you can see all bank cards</p>
            </div>
        )
    }



    return(
        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
            <div className="row mb-4">
                <div className="col">
                    <h1>Bank Cards</h1>

                    <div className="container">
                            <Lottie
                            options={animationOptions}
                            height={400}
                            width={400}
                            isStopped={false}
                            isPaused={false}
                            />
                        </div>
                    </div>
                </div>

                <div className="row">

                    <div className="col">
      
                {generateBankCardsView()}
                </div>
                </div>
            </div>
            
        </Layout>
    )
}


export default BankCards;