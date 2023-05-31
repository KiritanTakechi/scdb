<template>
    <div class="update">
        <div class="update__nav">
            <router-link class="update__nav__index" tag="div" :to="{ name: 'UpdateStudent' }">添加学生</router-link>
            <router-link class="update__nav__index" tag="div" :to="{ name: 'UpdateCourse' }">添加课程</router-link>
        </div>
        <div class="update__display">
            <router-view></router-view>
        </div>
    </div>
</template>
    
<script setup lang='ts'>

</script>
    
<style scoped>
.update {
    height: 100%;
    width: 100%;
}

.update__nav {
    height: auto;
    width: 100%;

    background-color: forestgreen;
    display: flex;
    justify-content: space-around;
}

.update__nav__index {
    padding: 5px;

    text-align: center;
    text-decoration: none;

    opacity: 0.8;

    color: white;

    font-weight: bold;
    font-size: small;
}

.nav-update__nav__index:hover {
    opacity: 1;
}

.update__display {
    height: 100%;
    width: 100%;
}
</style>