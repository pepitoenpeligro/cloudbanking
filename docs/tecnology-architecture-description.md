# Tecnology and architecture decissions



## Theory Introduction to Microservice Architecture.

The objective of this section is to motivate the use of an architecture based on microservices.

Microservices allow a software to be divided into small services, thus making it much easier to develop, debug, deploy and maintain.

To carry out the communication between the different microservices and these with the database, there are several alternatives such as REST, SOAP, AMQP or ZMQ.

Advantages of using microservices as opposed to a monolithic block:

1. It allows to be __polyglot__ and therefore, to develop each microservice in the language that is most convenient.
2. The code is __simplified and much smaller__. Updating and adding functionality is a more atomized and simple process than restructuring code in a monolithic architecture.
3. __Flexible (horizontal) scaling__. A development based on microservices allows to estimate costs and to scale according to the demand.
4. Work well within __containers__, such as Docker.
5. If __one service fails__, all __other services continue to work__. A partial drop of one functionality does not drag down the others.
6. Each __microservice has its own process__ (and culture, working with nodes, clusters and the cloud) of DevOps versus the role of system administrator (working with your own team).
7. If they run inside __containers__, they can be easily __orchestrated__ (docker-compose)
8. They are__ easily reusable__. For example, a microservice that provides authentication for one system is very easy to adapt for a different system.
9. Each microservice provides __minimum guarantees of isolation__, thus facilitating security issues and compatibility between microservices.
10. __Unit tests are simpler__ as the combination of functionality provided by the microservice in question is more packaged and defined.




Questions to be answered using microservices:

1. __Communication protocols__ must be designed between the microservices.
Not everything is a microservice. You have to know how to detect what a microservice is and what it is not. Too many microservices could overload the development of the software system we have in mind to build.
3. It is necessary to know __how to orchestrate__ well and correctly balance the load between the different microservices.
4. __Integration tests__ are more complex when we have to take into account each and every one of the microservices that provide the software system.



## Proposed design

Proposed microservices:
* Authentication microservice.
* Microservice for collecting statistics.
* Product catalog microservice (accounts, cards, deposits and common user savings accounts).
* Notification microservice (e-mails, push).