-- CreateTable
CREATE TABLE "Todo" (
    "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "describe" TEXT,

    CONSTRAINT "Todo_pkey" PRIMARY KEY ("id")
);
