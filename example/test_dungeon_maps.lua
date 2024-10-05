test_dungeon = {
{
9,9,
4,4,
1,1,1,1,1,1,1,1,1,
1,2,2,2,2,2,9,1,1,
1,2,1,2,1,1,4,1,1,
1,2,1,2,1,1,1,1,1,
1,2,1,2,3,2,2,2,1,
1,2,2,2,1,2,1,2,1,
1,2,1,1,1,2,1,2,1,
1,2,2,2,2,2,2,2,1,
1,1,1,1,1,1,1,1,1},
{
9,9,
4,4,
1,1,1,1,1,1,1,1,1,
1,2,2,2,2,2,1,1,1,
1,2,1,2,1,9,4,9,1,
1,2,1,2,1,1,1,2,1,
1,2,1,2,3,2,2,2,1,
1,2,2,2,1,1,2,1,1,
1,2,1,1,1,1,2,1,1,
1,2,2,2,2,2,2,1,1,
1,1,1,1,1,1,1,1,1}
}

test_dungeon_pointers = {}
totaloffset = 0
field("test_dungeon")
for i,d in ipairs(test_dungeon) do
    test_dungeon_pointers[i] = address()
   write(bytes(d))
end
field("test_dungeon_pointers")
write(bytes(test_dungeon_pointers, "uint24"))

test_dungeon_raycast = {
{
9,9,
4,4,
1,1,1,1,1,1,1,1,1,
1,0,0,0,0,0,0,4,1,
1,0,1,0,1,4,0,4,1,
1,0,1,0,3,3,4,4,1,
1,0,1,0,0,0,0,0,1,
1,0,0,0,3,0,1,0,1,
1,0,1,1,1,0,1,0,1,
1,0,0,0,0,0,0,0,1,
1,1,1,1,1,1,1,1,1},
{
9,9,
4,4,
1,1,1,1,1,1,1,1,1,
1,0,0,0,0,0,4,4,1,
1,0,1,0,1,0,0,0,1,
1,0,1,0,3,3,4,0,1,
1,0,1,0,0,0,0,0,1,
1,0,0,0,3,3,0,1,1,
1,0,1,1,1,1,0,1,1,
1,0,0,0,0,0,0,1,1,
1,1,1,1,1,1,1,1,1}
}

test_dungeon_raycast_pointers = {}
totaloffset = 0
field("test_dungeon_raycast")
for i,d in ipairs(test_dungeon_raycast) do
    test_dungeon_raycast_pointers[i] = address()
   write(bytes(d))
end
field("test_dungeon_raycast_pointers")
write(bytes(test_dungeon_raycast_pointers, "uint24"))


field("test_dungeon_pairs")
for i,d in ipairs(test_dungeon_raycast) do
    write(bytes({test_dungeon_pointers[i],test_dungeon_raycast_pointers[i]}, "uint24"))
end
    