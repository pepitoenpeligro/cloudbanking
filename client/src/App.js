import React from "react";
import Layout from "./core/Layout";

import "./App.css";

require("dotenv").config();

const App = () => {
  return (
	<Layout>
	  <div className="page-header mt-4 mb-4">
		<h1>Cloudbanking</h1>
	  </div>

	  <div>
		<div className="card mb-5 m-5 ">
		  <div className="card-header text-center h3">Notes for the user</div>
		  <div className="card-body">
			<div className="row">
			  <div className="col mb-4"></div>
			</div>
		  </div>
		</div>

		<div className="card mb-5 m-5 ">
		  <div className="card-header text-center h3">
			Notes for the developer
		  </div>
		  <div className="card-body">
			<div className="row">
			  <div className="col mb-4"></div>
			</div>
		  </div>
		</div>
	  </div>

	  {/* {isAuth() && welcome()} */}
	</Layout>
  );
};

export default App;
