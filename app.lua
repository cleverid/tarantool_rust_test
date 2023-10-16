box.cfg({ listen = 3301 })
box.schema.func.create('gant', { language = 'C', if_not_exists = true })
box.schema.user.grant('guest', 'execute', 'function', 'gant', { if_not_exists = true })
