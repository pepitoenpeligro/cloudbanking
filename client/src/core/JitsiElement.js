import React, { useState } from 'react'
 
import { Jutsu } from 'react-jutsu'
 
const JitsiElement = (props) => {
  const [room, setRoom] = useState('miroom')
  const [name, setName] = useState('Joselito')
  const [call, setCall] = useState(false)

  const [domain] = useState('jitsipepe.com')
  const [password] = useState('')
  const [parentNode] = useState('jitsi-container-'+props.index);
  const [index] = useState(props.index);

  const [jwt, setJwt] = useState(props.token);

  

  React.useEffect(() => {
    const strIn = props.token;
    const searchTerm = '?jwt=';
    const searchIndex = strIn.indexOf(searchTerm);
    const strOut = strIn.substr(searchIndex + searchTerm.length);
    
    console.log("Procesnado el jwt obtuve", strOut);

    let nameRoom = props.token;
    nameRoom = nameRoom.replace('https://jitsipepe.com/', '');
    console.log("Me queda", nameRoom)
    const searchIndex2 = nameRoom.indexOf('?');
    const strOutput2 = nameRoom.substr(0,searchIndex2);
    console.log("strOutpur2", strOutput2);

    // const JitsiMeetExternalAPI = window.JitsiMeetExternalAPI;



    setJwt(strOut);
    setRoom(strOutput2);
    setName(name);


    setCall(true);
  } ,[])

 
  const handleClick = event => {
    event.preventDefault()
    if (room && name) setCall(true)
  }
 
 return call ? 
(
      <div className="row">

          <Jutsu containerStyles={{ width: '100%', height: '800px' }}
      roomName={room}
      displayName={name}
      password={password}
      domain={domain}
      // domain={}
      parentNode={parentNode+index}
      jwt={jwt}
      loadingComponent={<p>loading ...</p>}  />

      </div>
    
      
  )


  
  
  : (
    
    <p>Error loading JitsiElement</p>

  )
}
 
export default JitsiElement