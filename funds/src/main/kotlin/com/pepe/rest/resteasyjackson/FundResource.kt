package com.pepe.rest.resteasyjackson
import javax.inject.Inject
import javax.enterprise.inject.Default
import javax.enterprise.context.ApplicationScoped
import javax.ws.rs.core.Response.*
import javax.ws.rs.core.Response
import org.jboss.logging.Logger


import java.util.*
import javax.ws.rs.*
import javax.ws.rs.core.MediaType
import java.time.LocalDateTime


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

    fun deleteFund(id: String): Boolean{
        funds.removeIf {  existingFund:    Fund -> existingFund.id!!.contentEquals(id!!) }
        return true
    }


}

@Path("/")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
class FundResource {

    companion object {
        private val LOG = Logger.getLogger(FundResource::class.java)

    }

    @Inject
    @field: Default 
    lateinit var service: FundService

    // Get all funds
    @GET
    @Produces(MediaType.APPLICATION_JSON)
    @Path("/funds")
    fun list():  Set<Fund> {
        LOG.info("GET -> returning funds")
        return service.getFunds()
    }

    // Creates one funds
    @POST
    @Path("/funds")
    fun  add(f: Fund): Response {
        val result = service.addFund(f)
        if(result){
            LOG.info("POST -> Fund was created")
            var m  = Message("Created", "Fund was created");
            return Response.status(Response.Status.ACCEPTED).entity(m).build();
        }
        LOG.info("POST -> Fund was NOT created")
        var m  = Message("Not Created", "Fund was not created");
        return Response.status(Response.Status.BAD_REQUEST).entity(m).build();

    }

    // Deletes one funds
    @DELETE
    @Path("/funds/{id}")
    fun delete(@PathParam("id") id: String): Boolean{
        LOG.info("DELETE -> Fund was deleted")
        return service.deleteFund(id)
    }

}