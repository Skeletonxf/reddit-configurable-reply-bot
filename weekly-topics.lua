local topics = {}

function topics.run()
  print('weekly topic identified')
  local topics = {
    ['August 26, 2018'] = "**The Weird Things Allosexuals Do**" .. newline ..
      "What things do allosexuals do that just confound you to no end?",
    ['September 2, 2018'] = "**Labels before there were labels**" .. newline ..
      "Before you discovered the term Demisexuality, did you go by a different label it descriptor? Did you try to fit into another label, or did you stand apart?",
    ['September 9, 2018'] = "**Man's Best Friends?**" .. newline ..
      "How is your relationship with animals when compared to people, easier, harder, about the same? Are animals something you can bond with other people over?",
    ['September 16, 2018'] = "**Freaky Friday**" .. newline ..
      "If you could live on the other side for one day/night, as an Allosexual, would you? What would you do?",
  }
  for week, topic in pairs(topics) do
    if containsIgnoreCase(title, week) then
      print('Posting Weekly topic')
      reply("" ..
        topic ..
        newline ..
        "*****" ..
        newline ..
        "Thanks to the Demisexuality discord for these topics https://discord.gg/4R8gKUa" ..
        footer
      )
    end
  end
end

return topics
