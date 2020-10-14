# PepitoBanking Initial Description

The problem to be solved with this project is the management and operation of different bank accounts, deposits and bank cards. The main idea is that a customer who has multiple financial products in different entities, can consult, manage and operate from a single platform.

In an initial description of the functionality, the platform should be able to: 
* Create an account in the service.
* Add a bank account
* Add a debit, credit or virtual card
* Add a deposit or investment fund.
* Explore the status of your products {bank account, cards and deposits}
* Set up alerts on your products (card purchase notification, account debit notification...)




## Arquitectura


### Arquitectura Software 

El proyecto va a seguir una arquitectura de microservicios.

* Un microservicio para la gestión de los usuarios (registro y autenticación de usuarios).
* Un microservicio para la gestión de los productos bancarios (añadir, explorar y operar).
* Un microservicio para la gestión de alertas sobre productos bancarios.

Para la codificación de estos microservicios, aún no lo tengo claro, estoy valorando opciones como Go, Ruby, Nodejs y Scala, porque son los lenguajes más ampliamente soportados hoy día en plataformas como Heroku , Amazon Elastic Beanstalk o Google App Engine.

### Comunicación e interacción de los componentes

La comunicación entre los diferentes microservicios será mediante API de tipo REST.


### Almacenamiento de datos.

El almacenamiento de datos será de tipo no relacional, es decir, que no requerirá estructuras fijas pero permita una gran escalabilidad horizontal, es decir, que baste con añadir más nodos en el clúster para que la capacidad de trabajo se vea incrementada y por tanto, el crecimiento del sistema de datos sea prácticamente infinito (siempre y cuando nuestro presupuesto nos lo permita).

Las opciones a barajar serán Redis, por ser una base de datos del tipo clave-valor que se adaptaría bastante bien a solucionar este problema, MongoDB, por permitir que cada documento tenga un esquema diferente, o CouchDB que permitiría realizar operaciones de tipo JOIN.


## Architecture


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




