void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // 1px = 1mm
    // 100px = 10cm
    // 1000px = 1m
    // speed of sound = 340 m/s = 340,000 mm/s
    // freqency = 440 Hz
    // wavelength = lambda = c/f = 340 m/s / 400Hz = 0.773 m
    //  = 773 mm = 773px
    vec2 speaker1 = vec2(400-50, 100)-fragCoord;
    vec2 speaker2 = vec2(400+50, 100)-fragCoord;
    float val1 = speaker1.x * speaker1.x + speaker1.y * speaker1.y;
    float val2 = speaker2.x * speaker2.x + speaker2.y * speaker2.y;
    val1 = sqrt(val1);
    val2 = sqrt(val2);
    float freq = 440.0 * 128.0;
    float lambda = 340000.0/freq;


    float result = sin(val1/lambda - iTime*3.0) + sin(val2/lambda - iTime*3.0);
    result = result/2.0;
    fragColor = vec4(result, result, result, 1.0);

}