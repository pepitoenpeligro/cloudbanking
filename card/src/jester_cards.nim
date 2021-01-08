

import json
import dotenv, os
import asyncdispatch, jester, strutils
import logging
import etcd_client
include ./jester_cards/controller

let  logger = newConsoleLogger()
let etcdClientObj = new_etcd_client(failover=false)

proc messageJson(msg: string): string =
  """{"msg": $#}""" % [msg]


let controller = CardController(bankCards: TableRef[string, Card]() )

router cloudbankingCardsRoutes:

  # Get a bank card by id
  get "/cards/@id":
    info("[GET] /cards/{id}")
    let paramReceived = @"id"
    let recoveredCard = controller.getBankCard(paramReceived)
    let idField = "" & recoveredCard.id;
    if idField == "-":
      error("Card not founded")
      resp(Http200, messageJson("Card not founded" & $(recoveredCard)), contentType="application/json")

    info("Card founded")
    resp(Http404, messageJson("Card founded: " & $(recoveredCard)), contentType="application/json")


  # HU 2: Create bank card
  post "/cards":
    info("[POST] /cards")
    let body = try: request.body.parseJson
               except: newJNull()

    if body.isNil:
      resp(Http400, messageJson("Invalid json"),
        contentType="application/json")

    let idjson : string = $ body["id"]
    let numberjson  : string = $ body["number"]
    let cvcjson : string = $body["cvc"];
    

    let newCard = Card(id: idjson,
                       number: numberjson,
                       cvc: cvcjson,
                       dateLimit: $ body["date_limit"],
                       status:  ($body["status"]).parseBool())

    let resultOperation = controller.addBankCard(newCard)
    if resultOperation:
      info("Card added successfully")
      resp(Http200,messageJson("Card added successfully"), contentType="application/json")
    else:
      error("Card NOT added successfully :" & ($resultOperation))
      resp(Http404,messageJson("Card NOT added successfully :" & ($resultOperation) ) , contentType="application/json")

  # HU 5: Delete bank card
  delete "/cards/@id":
    info("[GET] /cards/{id}")
    let paramReceived = @"id"
    let resultOperation: bool = controller.deleteBankCard(paramReceived)
    if resultOperation == true:
      info("Card was deleted successfully")
      resp(Http201, messageJson("Card was deleted successfully"), contentType="application/json")
    else: 
      error("Card was not found and not deleted")
      resp(Http404, messageJson("Card was not found and not deleted  successfully"  & ($resultOperation) ), contentType="application/json")





proc main() =
  var port:string = ""
  try:
    port = $etcdClientObj.get("PORT")
  except:
    echo("Could not found ETCD server")
  finally:
    echo("We are reading values from .env")
    let env = initDotEnv()
    env.load()

    port = os.getEnv("PORT")
    let settings = newSettings(port=port.parseInt().Port)
    var jester = initJester(cloudbankingCardsRoutes, settings=settings)
    jester.serve()

when isMainModule:
  main()
