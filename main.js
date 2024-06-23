async function tts(text, lang, options = {}) {
    const { utils } = options;
    const { tauriFetch } = utils;

    const res = await tauriFetch(`https://dict.youdao.com/dictvoice`,{
        method: "GET",
        query:{
            "audio":text,
            "le":lang
        },
        responseType: 3
    });

    if (res.ok) {
        let result = res.data;
        if (result) {
            return result;
        } else {
            throw JSON.stringify(result);
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}