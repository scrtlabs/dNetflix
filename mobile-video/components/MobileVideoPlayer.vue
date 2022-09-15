<template>
    <div class="video-player-container">
        <video ref="videoPlayer" class="player-style" fluid="true" controls></video>
        <div @click="close" class="close-button">Close Player</div>
    </div>
</template>

<script>
import Hls from 'hls.js';

export default {
    props: ['source', 'videoKey'],
    mounted() {
        this.$nextTick( async () => {
            // Get the manifest data so we can modify it
            let m3u8Data = await this.$http.$get(this.source);


            // Replace the dummy key with the URL of the echo server
            m3u8Data = m3u8Data.replace("key://0.key", `${process.env.NUXT_ENV_ECHO_VIDEO_SERVER}/GetVideoKey?key=${this.videoKey}`);
            
            // Since the part files won't be relative in the modified manifest, we need the full path for each video part
            var prefix = this.source.replace("main.m3u8", "");
            m3u8Data = m3u8Data.replaceAll("part_", `${prefix}part_`);

            var mimeType = "application/x-mpegurl";
            var src = `data:${mimeType};base64,${btoa(m3u8Data)}`;
            var video = this.$refs["videoPlayer"];

            // Check if the browser has native HLS support and if not, use HLS.js (can be also video.js id needed)
            if (video.canPlayType && video.canPlayType('application/x-mpegurl')) {
                video.src = src;
            } else {
                var hls = new Hls();
                hls.loadSource(src);
                hls.attachMedia(video);
            }
        });
    },
    computed: {
    },
    methods: {
        close() {
            this.$emit('close');
        },
    },
    data() {
        return {
        };
    }
};
</script>

<style scoped>
.video-player-container {
    width: 100% !important;
    z-index: 9010;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;
}

.video-player-container .close-button {
    cursor: pointer;
    z-index: 1007;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    font-weight: bold;
    /* bottom: -30px; */
    text-align: center;
    width: 100%;
    background-color: black;
}

.player-style {
    width: 100%;
    min-height: 300px;
}

</style>