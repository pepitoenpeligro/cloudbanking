# PepitoBanking Initial Description

El problema a resolver con este proyecto es el de la gestión y operación bancaria de diferentes cuentas bancarias, depósitos y tarjetas bancarias. La idea principal es que un cliente que posea múltiples productos financieros en entidades diferentes, puedan consultar, gestionar y operar desde una única plataforma.

En una descripción inicial de la funcionalidad, la plataforma debería de ser capaz de: 
* Crear una cuenta en el servicio.
* Añadir una cuenta bancaria
* Añadir una tarjeta de débito, crédito o virtual.
* Añadir un depósito o fondo de inversión.
* Explorar el estado de sus productos {cuenta bancaria, tarjetas y depósitos}
* Configurar alertas sobre sus productos (aviso de compras con tarjetas, aviso de adeudos con cuenta...)


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



