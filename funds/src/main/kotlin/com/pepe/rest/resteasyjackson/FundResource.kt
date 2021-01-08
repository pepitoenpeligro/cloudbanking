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

    fun addFund(f: Fund): Boolean{
        return funds.add(f)

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

    // Creates one funds
    @POST
    @Path("/funds")
    fun  add(f: Fund): Response {
        val result = service.addFund(f)
        if(result){
            println("POST -> Fund was created")
            var m  = Message("Created", "Fund was created");
            return Response.status(Response.Status.ACCEPTED).entity(m).build();
        }
        println("POST -> Fund was NOT created")
        var m  = Message("Not Created", "Fund was not created");
        return Response.status(Response.Status.BAD_REQUEST).entity(m).build();

    }


}