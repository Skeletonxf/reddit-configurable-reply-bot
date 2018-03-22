print 'hello from lua'

--print('the comment is "' .. comment .. '"')

askingIfDemi = false
for _, str in pairs({"am I demisexual", "if I'm demisexual", "make me demi?"}) do
    if containsIgnoreCase(comment, str) then
        askingIfDemi = true
    end
end

if askingIfDemi then
    print 'found someone asking if they are demi'
end

reply "I'm totally responding to the comment!"

--error 'crash the bot'
return true
