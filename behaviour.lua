print 'lua script running'

local newline = "\n\n"

local footer = newline ..
  "I'm a bot and this message was performed automatically. " ..
  "Contact /u/skeletonxf for issues. I'm also " ..
  "[open source](https://github.com/Skeletonxf/reddit-sexuality-definition-bot)"

askingIfDemi = false
for _, str in pairs({"am I demisexual", "if I'm demisexual", "make me demi?"}) do
    if containsIgnoreCase(comment or post, str) then
        askingIfDemi = true
    end
end

if askingIfDemi then
    print('found someone asking if they are demi: ' .. (comment or post))
end

if containsIgnoreCase(comment or post, "[[Demisexuality]]") then
  reply("A demisexual is a person who may experience sexual attraction but only after forming a strong emotion connection with that person(s)." ..
  "[Learn More](https://www.reddit.com/r/demisexuality/comments/7v2iwn/links_and_resources_masterpost/)" ..
  footer)
end


--error 'crash the bot'
--return true
