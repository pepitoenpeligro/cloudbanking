

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

    info ("Card founded")
    resp(Http404, messageJson("Card founded: " & $(recoveredCard)), contentType="application/json")







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
