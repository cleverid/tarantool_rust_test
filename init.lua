box.cfg({ listen = 3301 })

function functions()
    box.schema.func.create('app.insert', { language = 'C', if_not_exists = true })
    box.schema.user.grant('guest', 'execute', 'function', 'app.insert', { if_not_exists = true })
    box.schema.func.create('app.fiber_async', { language = 'C', if_not_exists = true })
    box.schema.user.grant('guest', 'execute', 'function', 'app.fiber_async', { if_not_exists = true })
end

function schema()
    local plan_item = box.schema.space.create('plan_item', { engine = 'vinyl' } )
    plan_item:format{
        { name = 'id', type = 'string' },
        { name = 'group_id', type = 'string' },
        { name = 'title', type = 'string' },
        { name = 'start_time', type = 'unsigned' },
        { name = 'end_time', type = 'unsigned' },
    }
    plan_item:create_index('primary', { type = 'TREE', parts = { 1, 'string' } })
    plan_item:create_index('group', { type = 'TREE', parts = { 2, 'string' } })

    local plan_dependence = box.schema.space.create('plan_dependence')
    plan_dependence:format{
        { name = 'id', type = 'string' },
        { name = 'predecessor_id', type = 'string' },
        { name = 'successor_id', type = 'string' },
    }
    plan_dependence:create_index('primary', { type = 'TREE', parts = { 1, 'string' } })
    plan_dependence:create_index('dependency', { type = 'TREE', parts = { 2, 'string', 3, 'string' } })
end

box.once('bootstrap_bench', function() 
    functions()
    schema()
end)
