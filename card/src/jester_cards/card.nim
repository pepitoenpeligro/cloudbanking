
type Card* = object 
    id*: string
    number*: string
    cvc*: string
    dateLimit*: string
    status*: bool

proc `==`(a: Card, b: Card): bool =
  return a.id != b.id

# proc getId*(c: Card): string = 
#     return c.id 

# proc setId*(c: var Card, id: string) = 
#     c.id = id


# proc getNumber*(c: Card): string = 
#     return c.number 

# proc setNumber*(c: var Card, number: string) = 
#     c.number = number


# proc getCVC*(c: Card): string = 
#     return c.cvc 

# proc setCVC*(c: var Card, cvc: string) = 
#     c.cvc = cvc


# proc getDateLimit*(c: Card): string = 
#     return c.dateLimit 

# proc setDateLimit*(c: var Card, dateLimit: string) = 
#     c.dateLimit = dateLimit


# proc getStatus(c: Card): bool = 
#     return c.status 

# proc setStatus(c: var Card, status: bool) = 
#     c.status = status