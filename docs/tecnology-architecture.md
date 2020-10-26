# Tecnology and architecture decissions


### Software Architecture 

The project will follow a microservices architecture.

* A microservice for user management (user registration and authentication).
* A microservice for the management of banking products (add, explore and operate).
* A microservice for managing alerts on bank products.

For the coding of these microservices, I am still not clear, I am evaluating options such as Go, Ruby, Nodejs and Scala, because they are the most widely supported languages today on platforms such as Heroku , Amazon Elastic Beanstalk or Google App Engine.

### Communication and interaction of the components

The communication between the different microservices will be through REST type API.


### Data storage.

The data storage will be of a non-relational type, that is, it will not require fixed structures but will allow a great horizontal scalability, that is, it will be enough to add more nodes in the cluster so that the work capacity will be increased and therefore, the growth of the data system will be practically infinite (as long as our budget allows it).

The options to shuffle will be Redis, because it is a key value database that would adapt quite well to solve this problem, MongoDB, for allowing each document to have a different scheme, or CouchDB that would allow JOIN type operations.