import React, {useState} from 'react'
import Layout from './Layout'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';
import * as RB from "react-bootstrap";
import Lottie from "react-lottie";




const BankAccounts = ({history}) => {

    const [values] = useState({
        accounts:[{
            "id": "999f7f66abf88ee70243988",
            "swift": "BSCHESMMXXX",
            "iban": "ES32668805111122223334",
            "created_at": "2020-07-08T22:18:00",
            "status": true
        },
        {
            "id": "999f7f66abf88ee70243000",
            "swift": "BSCHESMMXXX",
            "iban": "ES32668805111122223000",
            "created_at": "2020-07-08T22:18:00",
            "status": true
        }],
        accountsVisible:true
     });
 
    const {accounts, accountsVisible} = values;

    const animationOptions = {
        loop: true,
        autoplay: true,
        path: "https://assets4.lottiefiles.com/private_files/lf30_BU51PH.json"
        // height: 100,
        // rendererSettings: {
        //   preserveAspectRatio: "xMidYMid slice",
        // },
      };
 

     React.useEffect(() => {
        toast.error("This functionality will be completed in the next milestone. Sorry for the inconvenience");
    }, [])






    // const handleTestUnit = (item) =>  (event) => {
    //     history.push({pathname: '/testunits',state:item})
    //     console.log("EL boton ve:", item)
    // }

    const generateBankAccountView = (event) => {
        if (!accountsVisible) {
          return (
            <p>We are loading your accounts, please wait until this message changes</p>
          );
        } else {
          console.log("Estare bien?");
          console.log(accounts);
          return (
            <div>
              <RB.Table responsive>
                <thead>
                  <tr>
                    <th>ID</th>
                    <th>SWIFT</th>
                    <th>IBAN</th>
                    <th>Creation Date</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody>
                  {accounts.map((item) => (
                    <tr key={item.id}>
                      <td>{item.id}</td>
                      <td>{item.swift}</td>
                      <td>{item.iban}</td>
                      <td>{new Date(item.created_at).toLocaleDateString()}</td>
                      <td>{item.status + ""}</td>
                      {/* <td>{item.amount + '\t€'}</td>
                            <td>{item.duration + '\tdays'}</td> */}
    
                      {/* <td>{new Date(item.updatedAt).toLocaleDateString()}</td> */}
                    </tr>
                  ))}
                </tbody>
              </RB.Table>
            </div>
          );
        }
      };




    return(
        <Layout>
            <ToastContainer/>
            <div className="container mt-4 mb-4">
            <div className="row mb-4">
                <div className="col">
                    <h1>Bank Accounts</h1>
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
      
                {generateBankAccountView()}
                </div>
                </div>
            </div>
            
        </Layout>
    )
}


export default BankAccounts;