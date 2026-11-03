# Emberdeep — dungeon starter kit (design artifact)

## how_to_read

# How to read this artifact

So, the dungeon map -- here's the thing. the map is procedural but locked to seed 7: every reviewer and every future regeneration sees the same seven rooms. Plus, rooms sit in the north-east and south-west, so patrol routes cross the central corridor twice.

Let's talk the loot tables. weights make epics about 3% of drops, and the ledger above lists the exact drop counts this seed produces. Plus, the economy sheet derives its numbers from the same 400 generated rows, so tuning a weight regenerates every figure.

---
Rendered by spec2artifact — the map, the ledger and the prose cite the same data by construction.


## level

# Level 1 — Emberdeep Catacombs

################################################
###################....#########################
##......###########....#########################
##............................##################
##......###########....###.......###############
#####.#.#############.####.......###############
#####....############.####.......###############
#####....############.##########################
#####....############.#############........#####
#####....############.#####................#####
#####....#####....###.#####......##........#####
##############....###.#####......######.########
##############..........................########
##############....##############################
##############....##############################
################################################
################################################
################################################

# 7 rooms, 189 walkable cells of 864 (675 walls)


## level

# Level 1 — Emberdeep Catacombs

################################################
###################....#########################
##......###########....#########################
##............................##################
##......###########....###.......###############
#####.#.#############.####.......###############
#####....############.####.......###############
#####....############.##########################
#####....############.#############........#####
#####....############.#####................#####
#####....#####....###.#####......##........#####
##############....###.#####......######.########
##############..........................########
##############....##############################
##############....##############################
################################################
################################################
################################################

# 7 rooms, 189 walkable cells of 864 (675 walls)


## loot

```jsonl
{"id":1,"name":"Torn Cloak","category":"common","price":13.22}
{"id":2,"name":"Torn Cloak","category":"common","price":6.8}
{"id":3,"name":"Cracked Shield","category":"common","price":3.45}
{"id":4,"name":"Rusty Dagger","category":"common","price":20.02}
{"id":5,"name":"Cracked Shield","category":"common","price":45.06}
{"id":6,"name":"Rusty Dagger","category":"common","price":13.95}
{"id":7,"name":"Rusty Dagger","category":"common","price":20.37}
{"id":8,"name":"Cracked Shield","category":"common","price":14.62}
{"id":9,"name":"Wardweave Robe","category":"rare","price":28.48}
{"id":10,"name":"Steel Shortsword","category":"uncommon","price":16.95}
{"id":11,"name":"Wardweave Robe","category":"rare","price":25.96}
{"id":12,"name":"Cracked Shield","category":"common","price":8.2}
{"id":13,"name":"Wardweave Robe","category":"rare","price":28.31}
{"id":14,"name":"Torn Cloak","category":"common","price":14.44}
{"id":15,"name":"Heartflame Sigil","category":"epic","price":36.68}
{"id":16,"name":"Cracked Shield","category":"common","price":18.14}
{"id":17,"name":"Soul Sapphire","category":"rare","price":7.98}
{"id":18,"name":"Rusty Dagger","category":"common","price":36.53}
{"id":19,"name":"Soul Sapphire","category":"rare","price":12.08}
{"id":20,"name":"Rusty Dagger","category":"common","price":30.69}
{"id":21,"name":"Rusty Dagger","category":"common","price":40.18}
{"id":22,"name":"Bread Heel","category":"common","price":15.6}
{"id":23,"name":"Rusty Dagger","category":"common","price":8.9}
{"id":24,"name":"Emberbrand Blade","category":"rare","price":11.74}
{"id":25,"name":"Rusty Dagger","category":"common","price":13.16}
{"id":26,"name":"Bread Heel","category":"common","price":9.38}
{"id":27,"name":"Torn Cloak","category":"common","price":26.03}
{"id":28,"name":"Cracked Shield","category":"common","price":24.34}
{"id":29,"name":"Healing Draught","category":"uncommon","price":2.97}
{"id":30,"name":"Healing Draught","category":"uncommon","price":24.6}
{"id":31,"name":"Cracked Shield","category":"common","price":5.46}
{"id":32,"name":"Rusty Dagger","category":"common","price":12.28}
{"id":33,"name":"Steel Shortsword","category":"uncommon","price":8.26}
{"id":34,"name":"Torn Cloak","category":"common","price":7.33}
{"id":35,"name":"Rusty Dagger","category":"common","price":53.78}
{"id":36,"name":"Traveler's Amulet","category":"uncommon","price":15.07}
{"id":37,"name":"Rusty Dagger","category":"common","price":26.5}
{"id":38,"name":"Torn Cloak","category":"common","price":3.95}
{"id":39,"name":"Soul Sapphire","category":"rare","price":26.87}
{"id":40,"name":"Torn Cloak","category":"common","price":7.41}
{"id":41,"name":"Rusty Dagger","category":"common","price":6.2}
{"id":42,"name":"Cracked Shield","category":"common","price":11.08}
{"id":43,"name":"Steel Shortsword","category":"uncommon","price":6.41}
{"id":44,"name":"Bread Heel","category":"common","price":14.52}
{"id":45,"name":"Emberbrand Blade","category":"rare","price":15.11}
{"id":46,"name":"Steel Shortsword","category":"uncommon","price":4.27}
{"id":47,"name":"Cracked Shield","category":"common","price":32.76}
{"id":48,"name":"Traveler's Amulet","category":"uncommon","price":13.76}
{"id":49,"name":"Traveler's Amulet","category":"uncommon","price":41.6}
{"id":50,"name":"Healing Draught","category":"uncommon","price":7.07}
{"id":51,"name":"Cracked Shield","category":"common","price":6.1}
{"id":52,"name":"Bread Heel","category":"common","price":5.96}
{"id":53,"name":"Emberbrand Blade","category":"rare","price":5.29}
{"id":54,"name":"Bread Heel","category":"common","price":21.95}
{"id":55,"name":"Healing Draught","category":"uncommon","price":10.17}
{"id":56,"name":"Cracked Shield","category":"common","price":9.03}
{"id":57,"name":"Cracked Shield","category":"common","price":11.84}
{"id":58,"name":"Cracked Shield","category":"common","price":6.77}
{"id":59,"name":"Ashen Reaver","category":"epic","price":9.24}
{"id":60,"name":"Traveler's Amulet","category":"uncommon","price":41.37}
{"id":61,"name":"Cracked Shield","category":"common","price":19.35}
{"id":62,"name":"Cracked Shield","category":"common","price":38.52}
{"id":63,"name":"Torn Cloak","category":"common","price":9.86}
{"id":64,"name":"Torn Cloak","category":"common","price":5.74}
{"id":65,"name":"Torn Cloak","category":"common","price":12.41}
{"id":66,"name":"Emberbrand Blade","category":"rare","price":3.93}
{"id":67,"name":"Emberbrand Blade","category":"rare","price":41.77}
{"id":68,"name":"Steel Shortsword","category":"uncommon","price":8.34}
{"id":69,"name":"Wardweave Robe","category":"rare","price":21.99}
{"id":70,"name":"Torn Cloak","category":"common","price":22.22}
{"id":71,"name":"Cracked Shield","category":"common","price":10.45}
{"id":72,"name":"Cracked Shield","category":"common","price":7.07}
{"id":73,"name":"Heartflame Sigil","category":"epic","price":12.06}
{"id":74,"name":"Bread Heel","category":"common","price":18.92}
{"id":75,"name":"Torn Cloak","category":"common","price":6.54}
{"id":76,"name":"Cracked Shield","category":"common","price":9.91}
{"id":77,"name":"Rusty Dagger","category":"common","price":15.46}
{"id":78,"name":"Wardweave Robe","category":"rare","price":150.45}
{"id":79,"name":"Wardweave Robe","category":"rare","price":14.91}
{"id":80,"name":"Cracked Shield","category":"common","price":9.52}
{"id":81,"name":"Bread Heel","category":"common","price":4.6}
{"id":82,"name":"Torn Cloak","category":"common","price":57.43}
{"id":83,"name":"Healing Draught","category":"uncommon","price":17.98}
{"id":84,"name":"Bread Heel","category":"common","price":8.65}
{"id":85,"name":"Rusty Dagger","category":"common","price":4.43}
{"id":86,"name":"Wardweave Robe","category":"rare","price":56.7}
{"id":87,"name":"Cracked Shield","category":"common","price":13.21}
{"id":88,"name":"Healing Draught","category":"uncommon","price":13.75}
{"id":89,"name":"Traveler's Amulet","category":"uncommon","price":5.04}
{"id":90,"name":"Torn Cloak","category":"common","price":17.96}
{"id":91,"name":"Cracked Shield","category":"common","price":16.38}
{"id":92,"name":"Steel Shortsword","category":"uncommon","price":12.43}
{"id":93,"name":"Cracked Shield","category":"common","price":13.28}
{"id":94,"name":"Traveler's Amulet","category":"uncommon","price":11.05}
{"id":95,"name":"Bread Heel","category":"common","price":8.37}
{"id":96,"name":"Wardweave Robe","category":"rare","price":6.12}
{"id":97,"name":"Torn Cloak","category":"common","price":10.66}
{"id":98,"name":"Torn Cloak","category":"common","price":30.44}
{"id":99,"name":"Torn Cloak","category":"common","price":2.16}
{"id":100,"name":"Rusty Dagger","category":"common","price":8.3}
{"id":101,"name":"Bread Heel","category":"common","price":27.39}
{"id":102,"name":"Rusty Dagger","category":"common","price":10.23}
{"id":103,"name":"Traveler's Amulet","category":"uncommon","price":31.93}
{"id":104,"name":"Cracked Shield","category":"common","price":14.47}
{"id":105,"name":"Cracked Shield","category":"common","price":18.93}
{"id":106,"name":"Emberbrand Blade","category":"rare","price":16.58}
{"id":107,"name":"Rusty Dagger","category":"common","price":21.65}
{"id":108,"name":"Torn Cloak","category":"common","price":9.89}
{"id":109,"name":"Traveler's Amulet","category":"uncommon","price":16.92}
{"id":110,"name":"Bread Heel","category":"common","price":20.7}
{"id":111,"name":"Rusty Dagger","category":"common","price":10.89}
{"id":112,"name":"Wardweave Robe","category":"rare","price":8.09}
{"id":113,"name":"Traveler's Amulet","category":"uncommon","price":26.53}
{"id":114,"name":"Torn Cloak","category":"common","price":27.12}
{"id":115,"name":"Emberbrand Blade","category":"rare","price":35.66}
{"id":116,"name":"Rusty Dagger","category":"common","price":4.7}
{"id":117,"name":"Bread Heel","category":"common","price":3.68}
{"id":118,"name":"Rusty Dagger","category":"common","price":36.43}
{"id":119,"name":"Traveler's Amulet","category":"uncommon","price":11.41}
{"id":120,"name":"Emberbrand Blade","category":"rare","price":9.83}
{"id":121,"name":"Steel Shortsword","category":"uncommon","price":25.81}
{"id":122,"name":"Bread Heel","category":"common","price":11.51}
{"id":123,"name":"Traveler's Amulet","category":"uncommon","price":4.4}
{"id":124,"name":"Steel Shortsword","category":"uncommon","price":20.62}
{"id":125,"name":"Steel Shortsword","category":"uncommon","price":78.82}
{"id":126,"name":"Wardweave Robe","category":"rare","price":4.28}
{"id":127,"name":"Rusty Dagger","category":"common","price":39.83}
{"id":128,"name":"Torn Cloak","category":"common","price":16.5}
{"id":129,"name":"Wardweave Robe","category":"rare","price":11.19}
{"id":130,"name":"Cracked Shield","category":"common","price":16.07}
{"id":131,"name":"Bread Heel","category":"common","price":36.67}
{"id":132,"name":"Torn Cloak","category":"common","price":2.84}
{"id":133,"name":"Crown of the Deep","category":"epic","price":12.01}
{"id":134,"name":"Torn Cloak","category":"common","price":60.03}
{"id":135,"name":"Soul Sapphire","category":"rare","price":16.39}
{"id":136,"name":"Rusty Dagger","category":"common","price":7.34}
{"id":137,"name":"Rusty Dagger","category":"common","price":44.74}
{"id":138,"name":"Rusty Dagger","category":"common","price":3.0}
{"id":139,"name":"Bread Heel","category":"common","price":6.59}
{"id":140,"name":"Rusty Dagger","category":"common","price":9.38}
{"id":141,"name":"Wardweave Robe","category":"rare","price":15.11}
{"id":142,"name":"Heartflame Sigil","category":"epic","price":4.12}
{"id":143,"name":"Emberbrand Blade","category":"rare","price":12.09}
{"id":144,"name":"Bread Heel","category":"common","price":9.29}
{"id":145,"name":"Emberbrand Blade","category":"rare","price":10.22}
{"id":146,"name":"Emberbrand Blade","category":"rare","price":18.49}
{"id":147,"name":"Soul Sapphire","category":"rare","price":23.39}
{"id":148,"name":"Healing Draught","category":"uncommon","price":4.07}
{"id":149,"name":"Cracked Shield","category":"common","price":2.49}
{"id":150,"name":"Healing Draught","category":"uncommon","price":16.11}
{"id":151,"name":"Cracked Shield","category":"common","price":19.05}
{"id":152,"name":"Rusty Dagger","category":"common","price":9.73}
{"id":153,"name":"Steel Shortsword","category":"uncommon","price":15.4}
{"id":154,"name":"Torn Cloak","category":"common","price":4.93}
{"id":155,"name":"Torn Cloak","category":"common","price":15.33}
{"id":156,"name":"Healing Draught","category":"uncommon","price":2.0}
{"id":157,"name":"Rusty Dagger","category":"common","price":6.24}
{"id":158,"name":"Ashen Reaver","category":"epic","price":13.66}
{"id":159,"name":"Bread Heel","category":"common","price":4.37}
{"id":160,"name":"Bread Heel","category":"common","price":24.04}
{"id":161,"name":"Cracked Shield","category":"common","price":7.73}
{"id":162,"name":"Steel Shortsword","category":"uncommon","price":98.04}
{"id":163,"name":"Healing Draught","category":"uncommon","price":14.56}
{"id":164,"name":"Rusty Dagger","category":"common","price":44.97}
{"id":165,"name":"Torn Cloak","category":"common","price":19.56}
{"id":166,"name":"Traveler's Amulet","category":"uncommon","price":8.25}
{"id":167,"name":"Torn Cloak","category":"common","price":21.49}
{"id":168,"name":"Torn Cloak","category":"common","price":11.72}
{"id":169,"name":"Cracked Shield","category":"common","price":28.46}
{"id":170,"name":"Rusty Dagger","category":"common","price":10.3}
{"id":171,"name":"Wardweave Robe","category":"rare","price":14.96}
{"id":172,"name":"Rusty Dagger","category":"common","price":31.42}
{"id":173,"name":"Bread Heel","category":"common","price":1.81}
{"id":174,"name":"Cracked Shield","category":"common","price":15.81}
{"id":175,"name":"Crown of the Deep","category":"epic","price":20.93}
{"id":176,"name":"Steel Shortsword","category":"uncommon","price":18.54}
{"id":177,"name":"Emberbrand Blade","category":"rare","price":20.53}
{"id":178,"name":"Bread Heel","category":"common","price":7.56}
{"id":179,"name":"Steel Shortsword","category":"uncommon","price":50.78}
{"id":180,"name":"Traveler's Amulet","category":"uncommon","price":14.56}
{"id":181,"name":"Cracked Shield","category":"common","price":5.05}
{"id":182,"name":"Bread Heel","category":"common","price":10.97}
{"id":183,"name":"Rusty Dagger","category":"common","price":77.51}
{"id":184,"name":"Steel Shortsword","category":"uncommon","price":12.31}
{"id":185,"name":"Torn Cloak","category":"common","price":23.91}
{"id":186,"name":"Torn Cloak","category":"common","price":8.11}
{"id":187,"name":"Cracked Shield","category":"common","price":23.36}
{"id":188,"name":"Rusty Dagger","category":"common","price":53.68}
{"id":189,"name":"Cracked Shield","category":"common","price":59.53}
{"id":190,"name":"Torn Cloak","category":"common","price":25.71}
{"id":191,"name":"Bread Heel","category":"common","price":3.5}
{"id":192,"name":"Torn Cloak","category":"common","price":29.98}
{"id":193,"name":"Rusty Dagger","category":"common","price":7.68}
{"id":194,"name":"Cracked Shield","category":"common","price":18.74}
{"id":195,"name":"Bread Heel","category":"common","price":22.41}
{"id":196,"name":"Cracked Shield","category":"common","price":11.34}
{"id":197,"name":"Rusty Dagger","category":"common","price":20.58}
{"id":198,"name":"Torn Cloak","category":"common","price":4.69}
{"id":199,"name":"Bread Heel","category":"common","price":3.94}
{"id":200,"name":"Rusty Dagger","category":"common","price":7.0}
{"id":201,"name":"Bread Heel","category":"common","price":11.73}
{"id":202,"name":"Steel Shortsword","category":"uncommon","price":14.52}
{"id":203,"name":"Rusty Dagger","category":"common","price":9.91}
{"id":204,"name":"Cracked Shield","category":"common","price":9.03}
{"id":205,"name":"Steel Shortsword","category":"uncommon","price":24.19}
{"id":206,"name":"Rusty Dagger","category":"common","price":8.04}
{"id":207,"name":"Steel Shortsword","category":"uncommon","price":14.78}
{"id":208,"name":"Ashen Reaver","category":"epic","price":18.34}
{"id":209,"name":"Torn Cloak","category":"common","price":4.93}
{"id":210,"name":"Cracked Shield","category":"common","price":15.6}
{"id":211,"name":"Bread Heel","category":"common","price":41.39}
{"id":212,"name":"Rusty Dagger","category":"common","price":4.79}
{"id":213,"name":"Bread Heel","category":"common","price":72.06}
{"id":214,"name":"Steel Shortsword","category":"uncommon","price":23.46}
{"id":215,"name":"Cracked Shield","category":"common","price":5.94}
{"id":216,"name":"Ashen Reaver","category":"epic","price":9.42}
{"id":217,"name":"Rusty Dagger","category":"common","price":30.3}
{"id":218,"name":"Soul Sapphire","category":"rare","price":17.63}
{"id":219,"name":"Cracked Shield","category":"common","price":29.84}
{"id":220,"name":"Cracked Shield","category":"common","price":12.09}
{"id":221,"name":"Torn Cloak","category":"common","price":12.96}
{"id":222,"name":"Bread Heel","category":"common","price":5.89}
{"id":223,"name":"Torn Cloak","category":"common","price":10.21}
{"id":224,"name":"Rusty Dagger","category":"common","price":7.23}
{"id":225,"name":"Torn Cloak","category":"common","price":13.67}
{"id":226,"name":"Bread Heel","category":"common","price":17.38}
{"id":227,"name":"Healing Draught","category":"uncommon","price":10.96}
{"id":228,"name":"Rusty Dagger","category":"common","price":5.78}
{"id":229,"name":"Torn Cloak","category":"common","price":17.82}
{"id":230,"name":"Traveler's Amulet","category":"uncommon","price":9.6}
{"id":231,"name":"Cracked Shield","category":"common","price":38.02}
{"id":232,"name":"Rusty Dagger","category":"common","price":19.81}
{"id":233,"name":"Torn Cloak","category":"common","price":8.87}
{"id":234,"name":"Emberbrand Blade","category":"rare","price":3.74}
{"id":235,"name":"Cracked Shield","category":"common","price":13.45}
{"id":236,"name":"Rusty Dagger","category":"common","price":21.68}
{"id":237,"name":"Steel Shortsword","category":"uncommon","price":19.5}
{"id":238,"name":"Traveler's Amulet","category":"uncommon","price":4.72}
{"id":239,"name":"Traveler's Amulet","category":"uncommon","price":28.74}
{"id":240,"name":"Steel Shortsword","category":"uncommon","price":10.28}
{"id":241,"name":"Soul Sapphire","category":"rare","price":43.79}
{"id":242,"name":"Torn Cloak","category":"common","price":8.21}
{"id":243,"name":"Soul Sapphire","category":"rare","price":2.94}
{"id":244,"name":"Emberbrand Blade","category":"rare","price":7.22}
{"id":245,"name":"Steel Shortsword","category":"uncommon","price":19.39}
{"id":246,"name":"Traveler's Amulet","category":"uncommon","price":16.58}
{"id":247,"name":"Torn Cloak","category":"common","price":31.67}
{"id":248,"name":"Bread Heel","category":"common","price":2.37}
{"id":249,"name":"Bread Heel","category":"common","price":33.31}
{"id":250,"name":"Bread Heel","category":"common","price":5.13}
{"id":251,"name":"Rusty Dagger","category":"common","price":8.45}
{"id":252,"name":"Rusty Dagger","category":"common","price":11.99}
{"id":253,"name":"Steel Shortsword","category":"uncommon","price":18.71}
{"id":254,"name":"Bread Heel","category":"common","price":37.13}
{"id":255,"name":"Traveler's Amulet","category":"uncommon","price":9.49}
{"id":256,"name":"Steel Shortsword","category":"uncommon","price":3.33}
{"id":257,"name":"Rusty Dagger","category":"common","price":63.39}
{"id":258,"name":"Traveler's Amulet","category":"uncommon","price":40.09}
{"id":259,"name":"Soul Sapphire","category":"rare","price":20.63}
{"id":260,"name":"Wardweave Robe","category":"rare","price":41.84}
{"id":261,"name":"Rusty Dagger","category":"common","price":5.19}
{"id":262,"name":"Steel Shortsword","category":"uncommon","price":14.99}
{"id":263,"name":"Healing Draught","category":"uncommon","price":15.08}
{"id":264,"name":"Rusty Dagger","category":"common","price":3.22}
{"id":265,"name":"Bread Heel","category":"common","price":21.61}
{"id":266,"name":"Cracked Shield","category":"common","price":19.43}
{"id":267,"name":"Healing Draught","category":"uncommon","price":9.38}
{"id":268,"name":"Wardweave Robe","category":"rare","price":15.66}
{"id":269,"name":"Emberbrand Blade","category":"rare","price":6.21}
{"id":270,"name":"Torn Cloak","category":"common","price":34.0}
{"id":271,"name":"Steel Shortsword","category":"uncommon","price":13.19}
{"id":272,"name":"Torn Cloak","category":"common","price":18.95}
{"id":273,"name":"Rusty Dagger","category":"common","price":11.73}
{"id":274,"name":"Traveler's Amulet","category":"uncommon","price":15.05}
{"id":275,"name":"Torn Cloak","category":"common","price":84.71}
{"id":276,"name":"Torn Cloak","category":"common","price":26.22}
{"id":277,"name":"Rusty Dagger","category":"common","price":4.73}
{"id":278,"name":"Bread Heel","category":"common","price":26.87}
{"id":279,"name":"Cracked Shield","category":"common","price":6.83}
{"id":280,"name":"Cracked Shield","category":"common","price":19.31}
{"id":281,"name":"Emberbrand Blade","category":"rare","price":4.08}
{"id":282,"name":"Bread Heel","category":"common","price":12.85}
{"id":283,"name":"Cracked Shield","category":"common","price":2.04}
{"id":284,"name":"Wardweave Robe","category":"rare","price":29.14}
{"id":285,"name":"Torn Cloak","category":"common","price":14.19}
{"id":286,"name":"Emberbrand Blade","category":"rare","price":3.96}
{"id":287,"name":"Steel Shortsword","category":"uncommon","price":70.69}
{"id":288,"name":"Torn Cloak","category":"common","price":41.03}
{"id":289,"name":"Bread Heel","category":"common","price":7.36}
{"id":290,"name":"Bread Heel","category":"common","price":2.29}
{"id":291,"name":"Bread Heel","category":"common","price":5.82}
{"id":292,"name":"Traveler's Amulet","category":"uncommon","price":6.01}
{"id":293,"name":"Torn Cloak","category":"common","price":31.42}
{"id":294,"name":"Wardweave Robe","category":"rare","price":54.67}
{"id":295,"name":"Soul Sapphire","category":"rare","price":9.97}
{"id":296,"name":"Wardweave Robe","category":"rare","price":36.83}
{"id":297,"name":"Bread Heel","category":"common","price":19.21}
{"id":298,"name":"Emberbrand Blade","category":"rare","price":1.81}
{"id":299,"name":"Bread Heel","category":"common","price":5.37}
{"id":300,"name":"Soul Sapphire","category":"rare","price":11.76}
{"id":301,"name":"Healing Draught","category":"uncommon","price":81.99}
{"id":302,"name":"Steel Shortsword","category":"uncommon","price":22.4}
{"id":303,"name":"Rusty Dagger","category":"common","price":15.89}
{"id":304,"name":"Cracked Shield","category":"common","price":13.36}
{"id":305,"name":"Bread Heel","category":"common","price":13.56}
{"id":306,"name":"Rusty Dagger","category":"common","price":5.31}
{"id":307,"name":"Traveler's Amulet","category":"uncommon","price":26.77}
{"id":308,"name":"Healing Draught","category":"uncommon","price":25.38}
{"id":309,"name":"Torn Cloak","category":"common","price":13.48}
{"id":310,"name":"Steel Shortsword","category":"uncommon","price":18.03}
{"id":311,"name":"Traveler's Amulet","category":"uncommon","price":13.12}
{"id":312,"name":"Wardweave Robe","category":"rare","price":13.09}
{"id":313,"name":"Cracked Shield","category":"common","price":5.99}
{"id":314,"name":"Rusty Dagger","category":"common","price":13.95}
{"id":315,"name":"Torn Cloak","category":"common","price":20.22}
{"id":316,"name":"Bread Heel","category":"common","price":3.26}
{"id":317,"name":"Bread Heel","category":"common","price":4.36}
{"id":318,"name":"Torn Cloak","category":"common","price":10.47}
{"id":319,"name":"Rusty Dagger","category":"common","price":14.26}
{"id":320,"name":"Rusty Dagger","category":"common","price":15.16}
{"id":321,"name":"Steel Shortsword","category":"uncommon","price":8.12}
{"id":322,"name":"Rusty Dagger","category":"common","price":19.39}
{"id":323,"name":"Bread Heel","category":"common","price":4.74}
{"id":324,"name":"Steel Shortsword","category":"uncommon","price":20.36}
{"id":325,"name":"Cracked Shield","category":"common","price":22.36}
{"id":326,"name":"Torn Cloak","category":"common","price":18.49}
{"id":327,"name":"Bread Heel","category":"common","price":18.01}
{"id":328,"name":"Bread Heel","category":"common","price":15.86}
{"id":329,"name":"Traveler's Amulet","category":"uncommon","price":23.01}
{"id":330,"name":"Torn Cloak","category":"common","price":9.82}
{"id":331,"name":"Cracked Shield","category":"common","price":5.98}
{"id":332,"name":"Torn Cloak","category":"common","price":12.58}
{"id":333,"name":"Traveler's Amulet","category":"uncommon","price":3.02}
{"id":334,"name":"Cracked Shield","category":"common","price":3.29}
{"id":335,"name":"Cracked Shield","category":"common","price":42.32}
{"id":336,"name":"Cracked Shield","category":"common","price":21.15}
{"id":337,"name":"Cracked Shield","category":"common","price":39.23}
{"id":338,"name":"Wardweave Robe","category":"rare","price":1.77}
{"id":339,"name":"Bread Heel","category":"common","price":14.59}
{"id":340,"name":"Rusty Dagger","category":"common","price":34.98}
{"id":341,"name":"Cracked Shield","category":"common","price":3.41}
{"id":342,"name":"Cracked Shield","category":"common","price":8.31}
{"id":343,"name":"Torn Cloak","category":"common","price":7.39}
{"id":344,"name":"Soul Sapphire","category":"rare","price":29.54}
{"id":345,"name":"Cracked Shield","category":"common","price":11.79}
{"id":346,"name":"Torn Cloak","category":"common","price":32.7}
{"id":347,"name":"Bread Heel","category":"common","price":1.64}
{"id":348,"name":"Cracked Shield","category":"common","price":19.27}
{"id":349,"name":"Bread Heel","category":"common","price":15.63}
{"id":350,"name":"Rusty Dagger","category":"common","price":62.97}
{"id":351,"name":"Bread Heel","category":"common","price":7.17}
{"id":352,"name":"Bread Heel","category":"common","price":32.27}
{"id":353,"name":"Soul Sapphire","category":"rare","price":6.78}
{"id":354,"name":"Steel Shortsword","category":"uncommon","price":13.9}
{"id":355,"name":"Ashen Reaver","category":"epic","price":8.05}
{"id":356,"name":"Torn Cloak","category":"common","price":35.63}
{"id":357,"name":"Cracked Shield","category":"common","price":3.28}
{"id":358,"name":"Torn Cloak","category":"common","price":12.59}
{"id":359,"name":"Rusty Dagger","category":"common","price":18.36}
{"id":360,"name":"Cracked Shield","category":"common","price":17.52}
{"id":361,"name":"Rusty Dagger","category":"common","price":9.69}
{"id":362,"name":"Traveler's Amulet","category":"uncommon","price":13.48}
{"id":363,"name":"Torn Cloak","category":"common","price":4.06}
{"id":364,"name":"Traveler's Amulet","category":"uncommon","price":25.15}
{"id":365,"name":"Bread Heel","category":"common","price":25.97}
{"id":366,"name":"Torn Cloak","category":"common","price":8.44}
{"id":367,"name":"Traveler's Amulet","category":"uncommon","price":35.16}
{"id":368,"name":"Ashen Reaver","category":"epic","price":8.9}
{"id":369,"name":"Rusty Dagger","category":"common","price":32.53}
{"id":370,"name":"Wardweave Robe","category":"rare","price":5.36}
{"id":371,"name":"Cracked Shield","category":"common","price":16.12}
{"id":372,"name":"Traveler's Amulet","category":"uncommon","price":8.33}
{"id":373,"name":"Rusty Dagger","category":"common","price":10.3}
{"id":374,"name":"Emberbrand Blade","category":"rare","price":8.2}
{"id":375,"name":"Bread Heel","category":"common","price":9.42}
{"id":376,"name":"Cracked Shield","category":"common","price":10.14}
{"id":377,"name":"Rusty Dagger","category":"common","price":17.51}
{"id":378,"name":"Steel Shortsword","category":"uncommon","price":17.81}
{"id":379,"name":"Torn Cloak","category":"common","price":5.71}
{"id":380,"name":"Torn Cloak","category":"common","price":21.9}
{"id":381,"name":"Rusty Dagger","category":"common","price":9.36}
{"id":382,"name":"Healing Draught","category":"uncommon","price":66.97}
{"id":383,"name":"Torn Cloak","category":"common","price":10.81}
{"id":384,"name":"Rusty Dagger","category":"common","price":11.51}
{"id":385,"name":"Heartflame Sigil","category":"epic","price":26.17}
{"id":386,"name":"Bread Heel","category":"common","price":8.22}
{"id":387,"name":"Bread Heel","category":"common","price":54.75}
{"id":388,"name":"Wardweave Robe","category":"rare","price":20.21}
{"id":389,"name":"Steel Shortsword","category":"uncommon","price":19.45}
{"id":390,"name":"Bread Heel","category":"common","price":13.66}
{"id":391,"name":"Healing Draught","category":"uncommon","price":32.93}
{"id":392,"name":"Steel Shortsword","category":"uncommon","price":31.32}
{"id":393,"name":"Bread Heel","category":"common","price":8.77}
{"id":394,"name":"Healing Draught","category":"uncommon","price":32.86}
{"id":395,"name":"Torn Cloak","category":"common","price":22.36}
{"id":396,"name":"Steel Shortsword","category":"uncommon","price":15.77}
{"id":397,"name":"Emberbrand Blade","category":"rare","price":9.86}
{"id":398,"name":"Rusty Dagger","category":"common","price":9.58}
{"id":399,"name":"Traveler's Amulet","category":"uncommon","price":8.29}
{"id":400,"name":"Bread Heel","category":"common","price":14.65}
```

## distribution

Item price spread

[1.64,20.24)│████████████████████████████████████████ 283
[20.24,38.84)│████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 82
[38.84,57.44)│███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 22
[57.44,76.05)│█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 7
[76.05,94.65)│█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 4
[94.65,113.25)│░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1
[113.25,131.85)│░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0
[131.85,150.45)│░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1

## economy

# Economy sheet

## Loot economy (400 rolls)
- Total market value: 7283.58.
- Value incl. 15% vendor tax: 8376.12.
- Mean item price: 18.20895; priciest drop: 150.45.
- Expected value per 100 pulls: 1821.
- A typical pull is worth 18.2 — budget drops to keep the dungeon profitable.


## loot

```jsonl
{"id":1,"name":"Torn Cloak","category":"common","price":13.22}
{"id":2,"name":"Torn Cloak","category":"common","price":6.8}
{"id":3,"name":"Cracked Shield","category":"common","price":3.45}
{"id":4,"name":"Rusty Dagger","category":"common","price":20.02}
{"id":5,"name":"Cracked Shield","category":"common","price":45.06}
{"id":6,"name":"Rusty Dagger","category":"common","price":13.95}
{"id":7,"name":"Rusty Dagger","category":"common","price":20.37}
{"id":8,"name":"Cracked Shield","category":"common","price":14.62}
{"id":9,"name":"Wardweave Robe","category":"rare","price":28.48}
{"id":10,"name":"Steel Shortsword","category":"uncommon","price":16.95}
{"id":11,"name":"Wardweave Robe","category":"rare","price":25.96}
{"id":12,"name":"Cracked Shield","category":"common","price":8.2}
{"id":13,"name":"Wardweave Robe","category":"rare","price":28.31}
{"id":14,"name":"Torn Cloak","category":"common","price":14.44}
{"id":15,"name":"Heartflame Sigil","category":"epic","price":36.68}
{"id":16,"name":"Cracked Shield","category":"common","price":18.14}
{"id":17,"name":"Soul Sapphire","category":"rare","price":7.98}
{"id":18,"name":"Rusty Dagger","category":"common","price":36.53}
{"id":19,"name":"Soul Sapphire","category":"rare","price":12.08}
{"id":20,"name":"Rusty Dagger","category":"common","price":30.69}
{"id":21,"name":"Rusty Dagger","category":"common","price":40.18}
{"id":22,"name":"Bread Heel","category":"common","price":15.6}
{"id":23,"name":"Rusty Dagger","category":"common","price":8.9}
{"id":24,"name":"Emberbrand Blade","category":"rare","price":11.74}
{"id":25,"name":"Rusty Dagger","category":"common","price":13.16}
{"id":26,"name":"Bread Heel","category":"common","price":9.38}
{"id":27,"name":"Torn Cloak","category":"common","price":26.03}
{"id":28,"name":"Cracked Shield","category":"common","price":24.34}
{"id":29,"name":"Healing Draught","category":"uncommon","price":2.97}
{"id":30,"name":"Healing Draught","category":"uncommon","price":24.6}
{"id":31,"name":"Cracked Shield","category":"common","price":5.46}
{"id":32,"name":"Rusty Dagger","category":"common","price":12.28}
{"id":33,"name":"Steel Shortsword","category":"uncommon","price":8.26}
{"id":34,"name":"Torn Cloak","category":"common","price":7.33}
{"id":35,"name":"Rusty Dagger","category":"common","price":53.78}
{"id":36,"name":"Traveler's Amulet","category":"uncommon","price":15.07}
{"id":37,"name":"Rusty Dagger","category":"common","price":26.5}
{"id":38,"name":"Torn Cloak","category":"common","price":3.95}
{"id":39,"name":"Soul Sapphire","category":"rare","price":26.87}
{"id":40,"name":"Torn Cloak","category":"common","price":7.41}
{"id":41,"name":"Rusty Dagger","category":"common","price":6.2}
{"id":42,"name":"Cracked Shield","category":"common","price":11.08}
{"id":43,"name":"Steel Shortsword","category":"uncommon","price":6.41}
{"id":44,"name":"Bread Heel","category":"common","price":14.52}
{"id":45,"name":"Emberbrand Blade","category":"rare","price":15.11}
{"id":46,"name":"Steel Shortsword","category":"uncommon","price":4.27}
{"id":47,"name":"Cracked Shield","category":"common","price":32.76}
{"id":48,"name":"Traveler's Amulet","category":"uncommon","price":13.76}
{"id":49,"name":"Traveler's Amulet","category":"uncommon","price":41.6}
{"id":50,"name":"Healing Draught","category":"uncommon","price":7.07}
{"id":51,"name":"Cracked Shield","category":"common","price":6.1}
{"id":52,"name":"Bread Heel","category":"common","price":5.96}
{"id":53,"name":"Emberbrand Blade","category":"rare","price":5.29}
{"id":54,"name":"Bread Heel","category":"common","price":21.95}
{"id":55,"name":"Healing Draught","category":"uncommon","price":10.17}
{"id":56,"name":"Cracked Shield","category":"common","price":9.03}
{"id":57,"name":"Cracked Shield","category":"common","price":11.84}
{"id":58,"name":"Cracked Shield","category":"common","price":6.77}
{"id":59,"name":"Ashen Reaver","category":"epic","price":9.24}
{"id":60,"name":"Traveler's Amulet","category":"uncommon","price":41.37}
{"id":61,"name":"Cracked Shield","category":"common","price":19.35}
{"id":62,"name":"Cracked Shield","category":"common","price":38.52}
{"id":63,"name":"Torn Cloak","category":"common","price":9.86}
{"id":64,"name":"Torn Cloak","category":"common","price":5.74}
{"id":65,"name":"Torn Cloak","category":"common","price":12.41}
{"id":66,"name":"Emberbrand Blade","category":"rare","price":3.93}
{"id":67,"name":"Emberbrand Blade","category":"rare","price":41.77}
{"id":68,"name":"Steel Shortsword","category":"uncommon","price":8.34}
{"id":69,"name":"Wardweave Robe","category":"rare","price":21.99}
{"id":70,"name":"Torn Cloak","category":"common","price":22.22}
{"id":71,"name":"Cracked Shield","category":"common","price":10.45}
{"id":72,"name":"Cracked Shield","category":"common","price":7.07}
{"id":73,"name":"Heartflame Sigil","category":"epic","price":12.06}
{"id":74,"name":"Bread Heel","category":"common","price":18.92}
{"id":75,"name":"Torn Cloak","category":"common","price":6.54}
{"id":76,"name":"Cracked Shield","category":"common","price":9.91}
{"id":77,"name":"Rusty Dagger","category":"common","price":15.46}
{"id":78,"name":"Wardweave Robe","category":"rare","price":150.45}
{"id":79,"name":"Wardweave Robe","category":"rare","price":14.91}
{"id":80,"name":"Cracked Shield","category":"common","price":9.52}
{"id":81,"name":"Bread Heel","category":"common","price":4.6}
{"id":82,"name":"Torn Cloak","category":"common","price":57.43}
{"id":83,"name":"Healing Draught","category":"uncommon","price":17.98}
{"id":84,"name":"Bread Heel","category":"common","price":8.65}
{"id":85,"name":"Rusty Dagger","category":"common","price":4.43}
{"id":86,"name":"Wardweave Robe","category":"rare","price":56.7}
{"id":87,"name":"Cracked Shield","category":"common","price":13.21}
{"id":88,"name":"Healing Draught","category":"uncommon","price":13.75}
{"id":89,"name":"Traveler's Amulet","category":"uncommon","price":5.04}
{"id":90,"name":"Torn Cloak","category":"common","price":17.96}
{"id":91,"name":"Cracked Shield","category":"common","price":16.38}
{"id":92,"name":"Steel Shortsword","category":"uncommon","price":12.43}
{"id":93,"name":"Cracked Shield","category":"common","price":13.28}
{"id":94,"name":"Traveler's Amulet","category":"uncommon","price":11.05}
{"id":95,"name":"Bread Heel","category":"common","price":8.37}
{"id":96,"name":"Wardweave Robe","category":"rare","price":6.12}
{"id":97,"name":"Torn Cloak","category":"common","price":10.66}
{"id":98,"name":"Torn Cloak","category":"common","price":30.44}
{"id":99,"name":"Torn Cloak","category":"common","price":2.16}
{"id":100,"name":"Rusty Dagger","category":"common","price":8.3}
{"id":101,"name":"Bread Heel","category":"common","price":27.39}
{"id":102,"name":"Rusty Dagger","category":"common","price":10.23}
{"id":103,"name":"Traveler's Amulet","category":"uncommon","price":31.93}
{"id":104,"name":"Cracked Shield","category":"common","price":14.47}
{"id":105,"name":"Cracked Shield","category":"common","price":18.93}
{"id":106,"name":"Emberbrand Blade","category":"rare","price":16.58}
{"id":107,"name":"Rusty Dagger","category":"common","price":21.65}
{"id":108,"name":"Torn Cloak","category":"common","price":9.89}
{"id":109,"name":"Traveler's Amulet","category":"uncommon","price":16.92}
{"id":110,"name":"Bread Heel","category":"common","price":20.7}
{"id":111,"name":"Rusty Dagger","category":"common","price":10.89}
{"id":112,"name":"Wardweave Robe","category":"rare","price":8.09}
{"id":113,"name":"Traveler's Amulet","category":"uncommon","price":26.53}
{"id":114,"name":"Torn Cloak","category":"common","price":27.12}
{"id":115,"name":"Emberbrand Blade","category":"rare","price":35.66}
{"id":116,"name":"Rusty Dagger","category":"common","price":4.7}
{"id":117,"name":"Bread Heel","category":"common","price":3.68}
{"id":118,"name":"Rusty Dagger","category":"common","price":36.43}
{"id":119,"name":"Traveler's Amulet","category":"uncommon","price":11.41}
{"id":120,"name":"Emberbrand Blade","category":"rare","price":9.83}
{"id":121,"name":"Steel Shortsword","category":"uncommon","price":25.81}
{"id":122,"name":"Bread Heel","category":"common","price":11.51}
{"id":123,"name":"Traveler's Amulet","category":"uncommon","price":4.4}
{"id":124,"name":"Steel Shortsword","category":"uncommon","price":20.62}
{"id":125,"name":"Steel Shortsword","category":"uncommon","price":78.82}
{"id":126,"name":"Wardweave Robe","category":"rare","price":4.28}
{"id":127,"name":"Rusty Dagger","category":"common","price":39.83}
{"id":128,"name":"Torn Cloak","category":"common","price":16.5}
{"id":129,"name":"Wardweave Robe","category":"rare","price":11.19}
{"id":130,"name":"Cracked Shield","category":"common","price":16.07}
{"id":131,"name":"Bread Heel","category":"common","price":36.67}
{"id":132,"name":"Torn Cloak","category":"common","price":2.84}
{"id":133,"name":"Crown of the Deep","category":"epic","price":12.01}
{"id":134,"name":"Torn Cloak","category":"common","price":60.03}
{"id":135,"name":"Soul Sapphire","category":"rare","price":16.39}
{"id":136,"name":"Rusty Dagger","category":"common","price":7.34}
{"id":137,"name":"Rusty Dagger","category":"common","price":44.74}
{"id":138,"name":"Rusty Dagger","category":"common","price":3.0}
{"id":139,"name":"Bread Heel","category":"common","price":6.59}
{"id":140,"name":"Rusty Dagger","category":"common","price":9.38}
{"id":141,"name":"Wardweave Robe","category":"rare","price":15.11}
{"id":142,"name":"Heartflame Sigil","category":"epic","price":4.12}
{"id":143,"name":"Emberbrand Blade","category":"rare","price":12.09}
{"id":144,"name":"Bread Heel","category":"common","price":9.29}
{"id":145,"name":"Emberbrand Blade","category":"rare","price":10.22}
{"id":146,"name":"Emberbrand Blade","category":"rare","price":18.49}
{"id":147,"name":"Soul Sapphire","category":"rare","price":23.39}
{"id":148,"name":"Healing Draught","category":"uncommon","price":4.07}
{"id":149,"name":"Cracked Shield","category":"common","price":2.49}
{"id":150,"name":"Healing Draught","category":"uncommon","price":16.11}
{"id":151,"name":"Cracked Shield","category":"common","price":19.05}
{"id":152,"name":"Rusty Dagger","category":"common","price":9.73}
{"id":153,"name":"Steel Shortsword","category":"uncommon","price":15.4}
{"id":154,"name":"Torn Cloak","category":"common","price":4.93}
{"id":155,"name":"Torn Cloak","category":"common","price":15.33}
{"id":156,"name":"Healing Draught","category":"uncommon","price":2.0}
{"id":157,"name":"Rusty Dagger","category":"common","price":6.24}
{"id":158,"name":"Ashen Reaver","category":"epic","price":13.66}
{"id":159,"name":"Bread Heel","category":"common","price":4.37}
{"id":160,"name":"Bread Heel","category":"common","price":24.04}
{"id":161,"name":"Cracked Shield","category":"common","price":7.73}
{"id":162,"name":"Steel Shortsword","category":"uncommon","price":98.04}
{"id":163,"name":"Healing Draught","category":"uncommon","price":14.56}
{"id":164,"name":"Rusty Dagger","category":"common","price":44.97}
{"id":165,"name":"Torn Cloak","category":"common","price":19.56}
{"id":166,"name":"Traveler's Amulet","category":"uncommon","price":8.25}
{"id":167,"name":"Torn Cloak","category":"common","price":21.49}
{"id":168,"name":"Torn Cloak","category":"common","price":11.72}
{"id":169,"name":"Cracked Shield","category":"common","price":28.46}
{"id":170,"name":"Rusty Dagger","category":"common","price":10.3}
{"id":171,"name":"Wardweave Robe","category":"rare","price":14.96}
{"id":172,"name":"Rusty Dagger","category":"common","price":31.42}
{"id":173,"name":"Bread Heel","category":"common","price":1.81}
{"id":174,"name":"Cracked Shield","category":"common","price":15.81}
{"id":175,"name":"Crown of the Deep","category":"epic","price":20.93}
{"id":176,"name":"Steel Shortsword","category":"uncommon","price":18.54}
{"id":177,"name":"Emberbrand Blade","category":"rare","price":20.53}
{"id":178,"name":"Bread Heel","category":"common","price":7.56}
{"id":179,"name":"Steel Shortsword","category":"uncommon","price":50.78}
{"id":180,"name":"Traveler's Amulet","category":"uncommon","price":14.56}
{"id":181,"name":"Cracked Shield","category":"common","price":5.05}
{"id":182,"name":"Bread Heel","category":"common","price":10.97}
{"id":183,"name":"Rusty Dagger","category":"common","price":77.51}
{"id":184,"name":"Steel Shortsword","category":"uncommon","price":12.31}
{"id":185,"name":"Torn Cloak","category":"common","price":23.91}
{"id":186,"name":"Torn Cloak","category":"common","price":8.11}
{"id":187,"name":"Cracked Shield","category":"common","price":23.36}
{"id":188,"name":"Rusty Dagger","category":"common","price":53.68}
{"id":189,"name":"Cracked Shield","category":"common","price":59.53}
{"id":190,"name":"Torn Cloak","category":"common","price":25.71}
{"id":191,"name":"Bread Heel","category":"common","price":3.5}
{"id":192,"name":"Torn Cloak","category":"common","price":29.98}
{"id":193,"name":"Rusty Dagger","category":"common","price":7.68}
{"id":194,"name":"Cracked Shield","category":"common","price":18.74}
{"id":195,"name":"Bread Heel","category":"common","price":22.41}
{"id":196,"name":"Cracked Shield","category":"common","price":11.34}
{"id":197,"name":"Rusty Dagger","category":"common","price":20.58}
{"id":198,"name":"Torn Cloak","category":"common","price":4.69}
{"id":199,"name":"Bread Heel","category":"common","price":3.94}
{"id":200,"name":"Rusty Dagger","category":"common","price":7.0}
{"id":201,"name":"Bread Heel","category":"common","price":11.73}
{"id":202,"name":"Steel Shortsword","category":"uncommon","price":14.52}
{"id":203,"name":"Rusty Dagger","category":"common","price":9.91}
{"id":204,"name":"Cracked Shield","category":"common","price":9.03}
{"id":205,"name":"Steel Shortsword","category":"uncommon","price":24.19}
{"id":206,"name":"Rusty Dagger","category":"common","price":8.04}
{"id":207,"name":"Steel Shortsword","category":"uncommon","price":14.78}
{"id":208,"name":"Ashen Reaver","category":"epic","price":18.34}
{"id":209,"name":"Torn Cloak","category":"common","price":4.93}
{"id":210,"name":"Cracked Shield","category":"common","price":15.6}
{"id":211,"name":"Bread Heel","category":"common","price":41.39}
{"id":212,"name":"Rusty Dagger","category":"common","price":4.79}
{"id":213,"name":"Bread Heel","category":"common","price":72.06}
{"id":214,"name":"Steel Shortsword","category":"uncommon","price":23.46}
{"id":215,"name":"Cracked Shield","category":"common","price":5.94}
{"id":216,"name":"Ashen Reaver","category":"epic","price":9.42}
{"id":217,"name":"Rusty Dagger","category":"common","price":30.3}
{"id":218,"name":"Soul Sapphire","category":"rare","price":17.63}
{"id":219,"name":"Cracked Shield","category":"common","price":29.84}
{"id":220,"name":"Cracked Shield","category":"common","price":12.09}
{"id":221,"name":"Torn Cloak","category":"common","price":12.96}
{"id":222,"name":"Bread Heel","category":"common","price":5.89}
{"id":223,"name":"Torn Cloak","category":"common","price":10.21}
{"id":224,"name":"Rusty Dagger","category":"common","price":7.23}
{"id":225,"name":"Torn Cloak","category":"common","price":13.67}
{"id":226,"name":"Bread Heel","category":"common","price":17.38}
{"id":227,"name":"Healing Draught","category":"uncommon","price":10.96}
{"id":228,"name":"Rusty Dagger","category":"common","price":5.78}
{"id":229,"name":"Torn Cloak","category":"common","price":17.82}
{"id":230,"name":"Traveler's Amulet","category":"uncommon","price":9.6}
{"id":231,"name":"Cracked Shield","category":"common","price":38.02}
{"id":232,"name":"Rusty Dagger","category":"common","price":19.81}
{"id":233,"name":"Torn Cloak","category":"common","price":8.87}
{"id":234,"name":"Emberbrand Blade","category":"rare","price":3.74}
{"id":235,"name":"Cracked Shield","category":"common","price":13.45}
{"id":236,"name":"Rusty Dagger","category":"common","price":21.68}
{"id":237,"name":"Steel Shortsword","category":"uncommon","price":19.5}
{"id":238,"name":"Traveler's Amulet","category":"uncommon","price":4.72}
{"id":239,"name":"Traveler's Amulet","category":"uncommon","price":28.74}
{"id":240,"name":"Steel Shortsword","category":"uncommon","price":10.28}
{"id":241,"name":"Soul Sapphire","category":"rare","price":43.79}
{"id":242,"name":"Torn Cloak","category":"common","price":8.21}
{"id":243,"name":"Soul Sapphire","category":"rare","price":2.94}
{"id":244,"name":"Emberbrand Blade","category":"rare","price":7.22}
{"id":245,"name":"Steel Shortsword","category":"uncommon","price":19.39}
{"id":246,"name":"Traveler's Amulet","category":"uncommon","price":16.58}
{"id":247,"name":"Torn Cloak","category":"common","price":31.67}
{"id":248,"name":"Bread Heel","category":"common","price":2.37}
{"id":249,"name":"Bread Heel","category":"common","price":33.31}
{"id":250,"name":"Bread Heel","category":"common","price":5.13}
{"id":251,"name":"Rusty Dagger","category":"common","price":8.45}
{"id":252,"name":"Rusty Dagger","category":"common","price":11.99}
{"id":253,"name":"Steel Shortsword","category":"uncommon","price":18.71}
{"id":254,"name":"Bread Heel","category":"common","price":37.13}
{"id":255,"name":"Traveler's Amulet","category":"uncommon","price":9.49}
{"id":256,"name":"Steel Shortsword","category":"uncommon","price":3.33}
{"id":257,"name":"Rusty Dagger","category":"common","price":63.39}
{"id":258,"name":"Traveler's Amulet","category":"uncommon","price":40.09}
{"id":259,"name":"Soul Sapphire","category":"rare","price":20.63}
{"id":260,"name":"Wardweave Robe","category":"rare","price":41.84}
{"id":261,"name":"Rusty Dagger","category":"common","price":5.19}
{"id":262,"name":"Steel Shortsword","category":"uncommon","price":14.99}
{"id":263,"name":"Healing Draught","category":"uncommon","price":15.08}
{"id":264,"name":"Rusty Dagger","category":"common","price":3.22}
{"id":265,"name":"Bread Heel","category":"common","price":21.61}
{"id":266,"name":"Cracked Shield","category":"common","price":19.43}
{"id":267,"name":"Healing Draught","category":"uncommon","price":9.38}
{"id":268,"name":"Wardweave Robe","category":"rare","price":15.66}
{"id":269,"name":"Emberbrand Blade","category":"rare","price":6.21}
{"id":270,"name":"Torn Cloak","category":"common","price":34.0}
{"id":271,"name":"Steel Shortsword","category":"uncommon","price":13.19}
{"id":272,"name":"Torn Cloak","category":"common","price":18.95}
{"id":273,"name":"Rusty Dagger","category":"common","price":11.73}
{"id":274,"name":"Traveler's Amulet","category":"uncommon","price":15.05}
{"id":275,"name":"Torn Cloak","category":"common","price":84.71}
{"id":276,"name":"Torn Cloak","category":"common","price":26.22}
{"id":277,"name":"Rusty Dagger","category":"common","price":4.73}
{"id":278,"name":"Bread Heel","category":"common","price":26.87}
{"id":279,"name":"Cracked Shield","category":"common","price":6.83}
{"id":280,"name":"Cracked Shield","category":"common","price":19.31}
{"id":281,"name":"Emberbrand Blade","category":"rare","price":4.08}
{"id":282,"name":"Bread Heel","category":"common","price":12.85}
{"id":283,"name":"Cracked Shield","category":"common","price":2.04}
{"id":284,"name":"Wardweave Robe","category":"rare","price":29.14}
{"id":285,"name":"Torn Cloak","category":"common","price":14.19}
{"id":286,"name":"Emberbrand Blade","category":"rare","price":3.96}
{"id":287,"name":"Steel Shortsword","category":"uncommon","price":70.69}
{"id":288,"name":"Torn Cloak","category":"common","price":41.03}
{"id":289,"name":"Bread Heel","category":"common","price":7.36}
{"id":290,"name":"Bread Heel","category":"common","price":2.29}
{"id":291,"name":"Bread Heel","category":"common","price":5.82}
{"id":292,"name":"Traveler's Amulet","category":"uncommon","price":6.01}
{"id":293,"name":"Torn Cloak","category":"common","price":31.42}
{"id":294,"name":"Wardweave Robe","category":"rare","price":54.67}
{"id":295,"name":"Soul Sapphire","category":"rare","price":9.97}
{"id":296,"name":"Wardweave Robe","category":"rare","price":36.83}
{"id":297,"name":"Bread Heel","category":"common","price":19.21}
{"id":298,"name":"Emberbrand Blade","category":"rare","price":1.81}
{"id":299,"name":"Bread Heel","category":"common","price":5.37}
{"id":300,"name":"Soul Sapphire","category":"rare","price":11.76}
{"id":301,"name":"Healing Draught","category":"uncommon","price":81.99}
{"id":302,"name":"Steel Shortsword","category":"uncommon","price":22.4}
{"id":303,"name":"Rusty Dagger","category":"common","price":15.89}
{"id":304,"name":"Cracked Shield","category":"common","price":13.36}
{"id":305,"name":"Bread Heel","category":"common","price":13.56}
{"id":306,"name":"Rusty Dagger","category":"common","price":5.31}
{"id":307,"name":"Traveler's Amulet","category":"uncommon","price":26.77}
{"id":308,"name":"Healing Draught","category":"uncommon","price":25.38}
{"id":309,"name":"Torn Cloak","category":"common","price":13.48}
{"id":310,"name":"Steel Shortsword","category":"uncommon","price":18.03}
{"id":311,"name":"Traveler's Amulet","category":"uncommon","price":13.12}
{"id":312,"name":"Wardweave Robe","category":"rare","price":13.09}
{"id":313,"name":"Cracked Shield","category":"common","price":5.99}
{"id":314,"name":"Rusty Dagger","category":"common","price":13.95}
{"id":315,"name":"Torn Cloak","category":"common","price":20.22}
{"id":316,"name":"Bread Heel","category":"common","price":3.26}
{"id":317,"name":"Bread Heel","category":"common","price":4.36}
{"id":318,"name":"Torn Cloak","category":"common","price":10.47}
{"id":319,"name":"Rusty Dagger","category":"common","price":14.26}
{"id":320,"name":"Rusty Dagger","category":"common","price":15.16}
{"id":321,"name":"Steel Shortsword","category":"uncommon","price":8.12}
{"id":322,"name":"Rusty Dagger","category":"common","price":19.39}
{"id":323,"name":"Bread Heel","category":"common","price":4.74}
{"id":324,"name":"Steel Shortsword","category":"uncommon","price":20.36}
{"id":325,"name":"Cracked Shield","category":"common","price":22.36}
{"id":326,"name":"Torn Cloak","category":"common","price":18.49}
{"id":327,"name":"Bread Heel","category":"common","price":18.01}
{"id":328,"name":"Bread Heel","category":"common","price":15.86}
{"id":329,"name":"Traveler's Amulet","category":"uncommon","price":23.01}
{"id":330,"name":"Torn Cloak","category":"common","price":9.82}
{"id":331,"name":"Cracked Shield","category":"common","price":5.98}
{"id":332,"name":"Torn Cloak","category":"common","price":12.58}
{"id":333,"name":"Traveler's Amulet","category":"uncommon","price":3.02}
{"id":334,"name":"Cracked Shield","category":"common","price":3.29}
{"id":335,"name":"Cracked Shield","category":"common","price":42.32}
{"id":336,"name":"Cracked Shield","category":"common","price":21.15}
{"id":337,"name":"Cracked Shield","category":"common","price":39.23}
{"id":338,"name":"Wardweave Robe","category":"rare","price":1.77}
{"id":339,"name":"Bread Heel","category":"common","price":14.59}
{"id":340,"name":"Rusty Dagger","category":"common","price":34.98}
{"id":341,"name":"Cracked Shield","category":"common","price":3.41}
{"id":342,"name":"Cracked Shield","category":"common","price":8.31}
{"id":343,"name":"Torn Cloak","category":"common","price":7.39}
{"id":344,"name":"Soul Sapphire","category":"rare","price":29.54}
{"id":345,"name":"Cracked Shield","category":"common","price":11.79}
{"id":346,"name":"Torn Cloak","category":"common","price":32.7}
{"id":347,"name":"Bread Heel","category":"common","price":1.64}
{"id":348,"name":"Cracked Shield","category":"common","price":19.27}
{"id":349,"name":"Bread Heel","category":"common","price":15.63}
{"id":350,"name":"Rusty Dagger","category":"common","price":62.97}
{"id":351,"name":"Bread Heel","category":"common","price":7.17}
{"id":352,"name":"Bread Heel","category":"common","price":32.27}
{"id":353,"name":"Soul Sapphire","category":"rare","price":6.78}
{"id":354,"name":"Steel Shortsword","category":"uncommon","price":13.9}
{"id":355,"name":"Ashen Reaver","category":"epic","price":8.05}
{"id":356,"name":"Torn Cloak","category":"common","price":35.63}
{"id":357,"name":"Cracked Shield","category":"common","price":3.28}
{"id":358,"name":"Torn Cloak","category":"common","price":12.59}
{"id":359,"name":"Rusty Dagger","category":"common","price":18.36}
{"id":360,"name":"Cracked Shield","category":"common","price":17.52}
{"id":361,"name":"Rusty Dagger","category":"common","price":9.69}
{"id":362,"name":"Traveler's Amulet","category":"uncommon","price":13.48}
{"id":363,"name":"Torn Cloak","category":"common","price":4.06}
{"id":364,"name":"Traveler's Amulet","category":"uncommon","price":25.15}
{"id":365,"name":"Bread Heel","category":"common","price":25.97}
{"id":366,"name":"Torn Cloak","category":"common","price":8.44}
{"id":367,"name":"Traveler's Amulet","category":"uncommon","price":35.16}
{"id":368,"name":"Ashen Reaver","category":"epic","price":8.9}
{"id":369,"name":"Rusty Dagger","category":"common","price":32.53}
{"id":370,"name":"Wardweave Robe","category":"rare","price":5.36}
{"id":371,"name":"Cracked Shield","category":"common","price":16.12}
{"id":372,"name":"Traveler's Amulet","category":"uncommon","price":8.33}
{"id":373,"name":"Rusty Dagger","category":"common","price":10.3}
{"id":374,"name":"Emberbrand Blade","category":"rare","price":8.2}
{"id":375,"name":"Bread Heel","category":"common","price":9.42}
{"id":376,"name":"Cracked Shield","category":"common","price":10.14}
{"id":377,"name":"Rusty Dagger","category":"common","price":17.51}
{"id":378,"name":"Steel Shortsword","category":"uncommon","price":17.81}
{"id":379,"name":"Torn Cloak","category":"common","price":5.71}
{"id":380,"name":"Torn Cloak","category":"common","price":21.9}
{"id":381,"name":"Rusty Dagger","category":"common","price":9.36}
{"id":382,"name":"Healing Draught","category":"uncommon","price":66.97}
{"id":383,"name":"Torn Cloak","category":"common","price":10.81}
{"id":384,"name":"Rusty Dagger","category":"common","price":11.51}
{"id":385,"name":"Heartflame Sigil","category":"epic","price":26.17}
{"id":386,"name":"Bread Heel","category":"common","price":8.22}
{"id":387,"name":"Bread Heel","category":"common","price":54.75}
{"id":388,"name":"Wardweave Robe","category":"rare","price":20.21}
{"id":389,"name":"Steel Shortsword","category":"uncommon","price":19.45}
{"id":390,"name":"Bread Heel","category":"common","price":13.66}
{"id":391,"name":"Healing Draught","category":"uncommon","price":32.93}
{"id":392,"name":"Steel Shortsword","category":"uncommon","price":31.32}
{"id":393,"name":"Bread Heel","category":"common","price":8.77}
{"id":394,"name":"Healing Draught","category":"uncommon","price":32.86}
{"id":395,"name":"Torn Cloak","category":"common","price":22.36}
{"id":396,"name":"Steel Shortsword","category":"uncommon","price":15.77}
{"id":397,"name":"Emberbrand Blade","category":"rare","price":9.86}
{"id":398,"name":"Rusty Dagger","category":"common","price":9.58}
{"id":399,"name":"Traveler's Amulet","category":"uncommon","price":8.29}
{"id":400,"name":"Bread Heel","category":"common","price":14.65}
```

## rarity_chart

Drops by rarity

common   │████████████████████████████████████████████ 250
uncommon │███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 83
rare     │██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 55
epic     │██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 12

## rarity_pivot

# Rarity ledger

| category | avg price | drops | market value | signature item |
|---|---|---|---|---|
| common | 17.25724 | 250 | 4314.31 | Rusty Dagger |
| uncommon | 20.584337 | 83 | 1708.5 | Steel Shortsword |
| rare | 19.658 | 55 | 1081.19 | Wardweave Robe |
| epic | 14.965 | 12 | 179.58 | Ashen Reaver |

