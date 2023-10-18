box.cfg({listen = 3301})

box.schema.func.create('app.easy', {language = 'C', if_not_exists = true})
box.schema.func.create('app.easy2', {language = 'C', if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'app.easy', {if_not_exists = true})
box.schema.user.grant('guest', 'execute', 'function', 'app.easy2', {if_not_exists = true})
