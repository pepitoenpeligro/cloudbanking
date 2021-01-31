import React, { Component, useState } from 'react'
import Layout from '../core/Layout';
import axios from 'axios'
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.min.css';






const  StudentElement= (props) =>{


    const [values, setValues] = useState({
        name:props.data.name,
        email: props.data.email,
        userId: props.data._id,
        subjectId:props.idSubject,
        subjectName: props.nameSubject,
        linkGenerated:'https://null',
        textLink:'No Link',
        textLinkActive:false,
        btnemailstatus:true,
        indexValue: props.indice+1
        

    });

    const { name , email, subjectId, userId, subjectName, linkGenerated, textLink, btnemailstatus, indexValue, textLinkActive} = values;


    const handleGenerateLink = (event) => {
        // pedir a la api una clave para esta persona
        //console.log("pidiendo", values);
        console.log("pido:", {
            id: userId,
                name: name,
                email: email,
                roomname: subjectName
        });
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/generateroom`,
            data: {
                id: userId,
                name: name,
                email: email,
                roomname: subjectName
            },
            headers:{
               
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            setValues({...values, linkGenerated: response.data.domain, btnemailstatus: false,textLinkActive:true, textLink:'Go to Room'})
            toast.success(`Link generated for ${email}`);
        })
        .catch(function (error) {
           
            toast.error(`Error generating room for ${email}`);
            console.log(error);
        })

    }

    const handleSendEmail = (event) => {
        axios({
            method: 'POST',
            url: `${process.env.REACT_APP_API}/sendEmail`,
            data: {
                email: email,
                content: subjectName,
                link: linkGenerated
            },
            headers:{
               
                'Content-Type': 'application/json',
                'Accept':'application/json'
            }
        }).then(function (response) {
            setValues({...values, linkGenerated: response.data.domain, btnemailstatus: false, textLink:'Go to Room'})
            toast.success(`Email sendet to ${email}`);
            console.info(response);
        })
        .catch(function (error) {
           
            toast.error(`Error sending email for ${email}`);
            console.error(error);
        })

    }   

    
    return(
                 
        <tr>
            {console.log("El valor: ", indexValue)}
            <td>{indexValue}</td>
            <td>{name}</td>
            <td>{email}</td>
            {textLinkActive && (<td><a  href={linkGenerated}>{textLink}</a></td>) || <td>No link generated</td>}
            <td><button disabled={btnemailstatus?true:false} className="btn btn-primary" onClick={(handleSendEmail)}>Send Email</button></td>
            <td><button className="btn btn-primary" onClick={(handleGenerateLink)} >(re)Generate link</button></td>
        </tr>
        
        
        


    )
    
}

export default StudentElement;

