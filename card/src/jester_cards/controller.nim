import tables, hashes


from ./card import Card


type CardController* = object 
    bankCards*: TableRef[string, Card]


proc hash(mg: Card): Hash =
    return hash(mg.id)

proc addBankCard(cc: CardController, c:Card):bool=
    let founded = cc.bankCards.hasKey(c.id)
    if founded == false:
        let lastKey : string = c.id
        cc.bankCards[c.id] =  c
        return true
    else:
        return false

proc getBankCards*(cc: CardController): TableRef[string, Card] =
    return cc.bankCards

proc getBankCard*(cc: CardController, i: string) : Card = 
    let rule: string = "\"" & i & "\"";
    let founded = cc.bankCards.contains(rule)
    if founded==true:
        return cc.bankCards[rule]
    else:
        let emptyCard  = Card(id: "-", number:"-", cvc:"-", dateLimit:"-", status: false)
        return emptyCard

proc deleteBankCard*(cc: CardController, i: string): bool = 
    let rule: string = "\"" & i & "\"";
    let founded = cc.bankCards.contains(rule)
    if(founded):
        cc.bankCards.del(rule)
    return founded