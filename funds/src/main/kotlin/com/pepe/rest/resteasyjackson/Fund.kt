package com.pepe.rest.resteasyjackson

class Fund{
    var id: String? = null
    var amount: Float? =  0.0f
    var dateStart: String? = null
    var dateEnd:   String? = null
    var status: Boolean? = false

    constructor() {}

    constructor(id: String?, amount: Float?, dateStart: String?, dateEnd: String?, status: Boolean?){
        this.id = id
        this.amount = amount
        this.dateStart = dateStart
        this.dateEnd = dateEnd
        this.status = status
    }
}

