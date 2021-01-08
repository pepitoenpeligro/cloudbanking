package com.pepe.rest.resteasyjackson
import javax.inject.Inject
import javax.enterprise.inject.Default
import javax.enterprise.context.ApplicationScoped
import javax.ws.rs.core.Response.*
import javax.ws.rs.core.Response
import mu.KotlinLogging


import java.util.*
import java.time.LocalDateTime

import javax.ws.rs.GET
import javax.ws.rs.POST
import javax.ws.rs.DELETE
import javax.ws.rs.Path
import javax.ws.rs.PathParam
import javax.ws.rs.Produces
import javax.ws.rs.core.MediaType

private val logger = KotlinLogging.logger {} 

// ./mvnw compile quarkus:dev
@ApplicationScoped
class FundService {

    private val funds = Collections.newSetFromMap(Collections.synchronizedMap(LinkedHashMap<Fund, Boolean>()))


    fun greeting(name: String): String {
        return "hello $name"
    }

    fun getFunds(): Set<Fund>{
        return funds
    }


}

@Path("/")
class FundResource {

    @Inject
    @field: Default 
    lateinit var service: FundService

    // Get all funds
    @GET
    @Produces(MediaType.APPLICATION_JSON)
    @Path("/funds")
    fun list():  Set<Fund> {
        logger.debug { "GET -> returning funds" }
        println("GET -> returning funds")
        return service.getFunds()
    }

}