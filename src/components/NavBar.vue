<template>
    <div class="nav-bar">
        <router-link class="nav-bar__index" tag="div" :to="{ name: 'Index' }">首页</router-link>
        <router-link class="nav-bar__index" tag="div" :to="{ name: 'StudentList' }">学生列表</router-link>
        <router-link class="nav-bar__index" tag="div" :to="{ name: 'CourseList' }">课程列表</router-link>
        <router-link class="nav-bar__index" tag="div" :to="{ name: 'UpdateHome' }">新增数据</router-link>
    </div>
</template>
    
<script setup lang='ts'>

</script>
    
<style scoped>
.nav-bar {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
}

.nav-bar__index {
    padding: 10px;

    gap: 5px;

    text-align: center;
    text-decoration: none;

    /* background-color: #004469; */
    opacity: 0.8;

    color: white;

    font-weight: bolder;

}

.nav-bar__index:hover {
    opacity: 1;
}
</style>