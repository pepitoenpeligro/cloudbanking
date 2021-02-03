import React from "react";
import Layout from "./core/Layout";

import "./App.css";
import Lottie from "react-lottie";

require("dotenv").config();

const animationOptions = {
	loop: true,
	autoplay: true,
	path: "https://assets2.lottiefiles.com/packages/lf20_OdVhgq.json"
	// height: 100,
	// rendererSettings: {
	//   preserveAspectRatio: "xMidYMid slice",
	// },
  };

const App = () => {
  return (
	<Layout>
	  <div className="page-header mt-4 mb-4">
		<h1>Cloudbanking</h1>

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

	  <div>


		<div className="card mb-5 m-5 ">
		  <div className="card-header text-center h3">Notes for the user</div>
		  <div className="card-body">
			<div className="row">
			  <div></div>
			  <ul>
				  <li>You can delete all notifications with the bottom right button (trash can).</li>
				  <li>Funds and Loans are completly developed</li>
				  <li>Accounts and cards are in development</li>
				  <li>To start click on Signin please</li>
			  </ul>
			</div>
		  </div>
		</div>


	  
	  </div>

	  {/* {isAuth() && welcome()} */}
	</Layout>
  );
};

export default App;
