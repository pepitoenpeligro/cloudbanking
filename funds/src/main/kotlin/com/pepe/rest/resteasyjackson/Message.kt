package com.pepe.rest.resteasyjackson

class Message{
    var msg: String? = null
    var details: String? = null

    constructor() {}

    constructor(msg: String?, details: String?){
        this.msg = msg
        this.details = details
    }
}

