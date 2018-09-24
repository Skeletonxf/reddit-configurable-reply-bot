local topics = {}

function topics.run()
  print('weekly topic identified')
  local topics = {
    ['August 26, 2018'] = "**The Weird Things Allosexuals Do**" .. newline ..
      "What things do allosexuals do that just confound you to no end?",
    ['September 02, 2018'] = "**Labels before there were labels**" .. newline ..
      "Before you discovered the term Demisexuality, did you go by a different label it descriptor? Did you try to fit into another label, or did you stand apart?",
    ['September 09, 2018'] = "**Man's Best Friends?**" .. newline ..
      "How is your relationship with animals when compared to people, easier, harder, about the same? Are animals something you can bond with other people over?",
    ['September 16, 2018'] = "**Freaky Friday**" .. newline ..
      "If you could live on the other side for one day/night, as an Allosexual, would you? What would you do?",
    ['September 23, 2018'] = "**Loud And Proud**" .. newline ..
      "Do you tell people you're Demisexual, do people in general know what it means?",
    ['September 30, 2018'] = "**Gossip Girl**" .. newline ..
      "How do you feel when friends talk about their relationships and encounters? Do you join in, sit back and listen, or try to change the topic of conversation?",
    ['October 07, 2018'] = "**Shades Of Gray**" .. newline ..
      "What do you see as the difference between Gray-Asexual and Demisexual, being as they are so closely related?" .. newline ..
      "http://wiki.asexuality.org/Gray-A/Grey-A",
    ['October 14, 2018'] = "**A Demisexual Mascot**" .. newline ..
      'Some in the Asexual community see Dragons as their "Mascot" of sorts. If Demisexuals had a "Mascot", what would it be?',
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
