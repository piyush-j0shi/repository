from openai import OpenAI, completions

client = OpenAI(
            base_url = "https://integrate.api.nvidia.com/v1",
            api_key = "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"
        )

print("\n")
question = input("ask a question : ")

completion = client.chat.completions.create(
            model = "nvidia/llama-3.1-nemotron-70b-instruct",
            messages = [
                {
                    "role" : "system" , 
                    "content" : f"""
                            You need to create the poems according to the user question but Use the poem theme according to the Question.
                            Examples : if the question is 'wrote a poem about happy life' -> then poem theme should be light, clam and etc.
                            Now you got the idea so write the poem according to the user theme.
                            User question is {question}
                        """
                    }
                ],
            temperature = 0.5,
            top_p = 1,
            max_tokens = 1024,
            stream = True                        
        )

print("\n\nanswer : \n\n")

for chunk in completion:
    if chunk.choices[0].delta.content is not None:
        print(chunk.choices[0].delta.content, end = "")
