#[cfg(not(any(feature = "type_a_alternative_faster_lut")))]
#[rustfmt::skip]
// Original Waveforms from Waveshare
pub(crate) const LUT_FULL_UPDATE: [u8; 30] =[
    0x02, 0x02, 0x01, 0x11, 0x12, 0x12, 0x22, 0x22, 
    0x66, 0x69, 0x69, 0x59, 0x58, 0x99, 0x99, 0x88, 
    0x00, 0x00, 0x00, 0x00, 0xF8, 0xB4, 0x13, 0x51, 
    0x35, 0x51, 0x51, 0x19, 0x01, 0x00       
];

#[rustfmt::skip]
pub(crate) const LUT_PARTIAL_UPDATE: [u8; 30] =[
    0x10, 0x18, 0x18, 0x08, 0x18, 0x18, 0x08, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x13, 0x14, 0x44, 0x12, 
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00      
];

#[cfg(feature = "type_a_alternative_faster_lut")]
#[rustfmt::skip]
// Waveform from TeXiToi/il3820
pub(crate) const LUT_FULL_UPDATE: [u8; 30] =[
    0x50, 0xAA, 0x55, 0xAA, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
