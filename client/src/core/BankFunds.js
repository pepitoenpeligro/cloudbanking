import React, {useState} from "react";
import Layout from "./Layout";
import axios from "axios";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.min.css";
import * as RB from "react-bootstrap";
import Lottie from "react-lottie";

const BankFunds = ({ history }) => {
  const [values, setValues] = useState({
	funds: [],
	fundsVisible: false,
  });

  const animationOptions = {
	loop: true,
	autoplay: true,
	path: "https://assets4.lottiefiles.com/packages/lf20_q9mzugao.json"
	// height: 100,
	// rendererSettings: {
	//   preserveAspectRatio: "xMidYMid slice",
	// },
  };

  const {funds, fundsVisible} = values;

  React.useEffect(() => {
	axios({
	  method: "GET",
	  url: `${process.env.REACT_APP_API}/funds`,
	  headers: {
		"Content-Type": "application/json",
		Accept: "application/json",
	  },
	}).then(function (response) {
	  console.log(response);
	  console.log(response.data);
	  setValues((values) => ({
		...values,
		funds: response.data,
		fundsVisible: true,
	  }));
	  console.log("Funds object");
	  console.log(funds);
	  toast.success("Your bank funds have been recovered");
	});
	// eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

//   const handleTestUnit = (item) => (event) => {
// 	history.push({ pathname: "/testunits", state: item });
// 	console.log("EL boton ve:", item);
//   };


  const generateBankFundsView = (event) => {
	if (!fundsVisible) {
	  return (
		<p>We are loading your funds, please wait until this message changes</p>
	  );
	} else {
	  console.log("Estare bien?");
	  console.log(funds);
	  return (
		<div>
		  <RB.Table responsive>
			<thead>
			  <tr>
				<th>ID</th>
				<th>Amount</th>
				<th>Date Start</th>
				<th>Date End</th>
				<th>status</th>
			  </tr>
			</thead>
			<tbody>
			  {funds.map((item) => (
				<tr key={item.id}>
				  <td>{item.id}</td>
				  <td>{item.amount + "\t€"}</td>
				  <td>{item.dateStart}</td>
				  <td>{item.dateEnd}</td>
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

  return (
	<Layout>
	  <ToastContainer />
	  <div className="container mt-4 mb-4">


		<div className="row mb-4">
		  <div className="col">
			<h1>Bank Funds</h1>

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
		  <div className="col">{generateBankFundsView()}</div>
		</div>
	  </div>
	</Layout>
  );
};

export default BankFunds;
